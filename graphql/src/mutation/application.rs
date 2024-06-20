use super::{results, UserError};
use crate::webhooks;
use async_graphql::{Context, ErrorExtensions, InputObject, Object, Result, ResultExt};
use context::{checks, UserRole};
use database::{Application, ApplicationStatus, DraftApplication, Email, PgPool};
use std::sync::Arc;
use svix::api::Svix;
use tracing::{error, instrument};

results! {
    SubmitApplicationResult {
        /// The submitted application
        application: Application,
    }
    ChangeApplicationStatusResult {
        /// The updated application
        application: Application,
    }
    UpdateApplicationResult {
        /// The updated application
        application: Application,
    }
}

#[derive(Default)]
pub(crate) struct Mutation;

#[Object(name = "ApplicationMutation")]
impl Mutation {
    /// Submit a draft application
    #[instrument(name = "Mutation::submit_application", skip_all)]
    async fn submit_application(&self, ctx: &Context<'_>) -> Result<SubmitApplicationResult> {
        let user = checks::is_authenticated(ctx)?;
        let scope = checks::is_event(ctx)?;
        checks::has_role(ctx, UserRole::Participant)?;

        let db = ctx.data_unchecked::<PgPool>();
        let mut txn = db.begin().await?;
        if Application::exists(&scope.event, user.id, &mut txn)
            .await
            .extend()?
        {
            return Ok(
                UserError::new(&["submitApplication"], "application already submitted").into(),
            );
        }

        let application = match Application::from_draft(&scope.event, user.id, &mut txn).await {
            Ok(application) => application,
            Err(err) => {
                match err.as_ref() {
                    database::SqlxError::Database(err) => {
                        if matches!(err.code().as_deref(), Some("23502")) {
                            return Ok(UserError::new(
                                &["submitApplication"],
                                "application is incomplete",
                            )
                            .into());
                        }
                    }
                    database::SqlxError::RowNotFound => {
                        return Ok(UserError::new(
                            &["submitApplication"],
                            "could not find a draft application",
                        )
                        .into())
                    }
                    _ => {}
                }

                return Err(err.extend());
            }
        };

        DraftApplication::delete(&scope.event, user.id, &mut txn)
            .await
            .extend()?;

        txn.commit().await?;

        let svix = ctx.data_unchecked::<Arc<Svix>>();
        webhooks::send(svix, "application.submitted", &scope.event, &application).await;

        let mail = ctx.data_unchecked::<mail::Client>().clone();
        let email = user.email.clone();
        tokio::task::spawn(async move {
            if let Err(error) = mail.send_templated("pending", &email).await {
                error!(%error, "failed to send email")
            }
        });

        Ok(application.into())
    }

    /// Update an application's information
    ///
    /// The information that can be updated depends on the requester's role. For organizers and
    /// greater, only the flagged and notes fields can be updated.
    #[instrument(name = "Mutation::update_application", skip(self, ctx))]
    async fn update_application(
        &self,
        ctx: &Context<'_>,
        input: UpdateApplicationInput,
    ) -> Result<UpdateApplicationResult> {
        let scope = checks::is_event(ctx)?;
        checks::has_at_least_role(ctx, UserRole::Organizer)?;

        let db = ctx.data_unchecked::<PgPool>();
        let Some(mut application) = Application::find(&scope.event, input.id, db)
            .await
            .extend()?
        else {
            return Ok(UserError::new(&["id"], "application not found").into());
        };

        application
            .update()
            .override_flagged(input.flagged)
            .override_notes(input.notes)
            .save(db)
            .await
            .extend()?;

        Ok(application.into())
    }

    /// Change an application's status
    ///
    /// The following transitions are allowed:
    /// - PENDING    -> WAITLISTED, ACCEPTED, REJECTED
    /// - WAITLISTED -> ACCEPTED, REJECTED
    /// - ACCEPTED   -> ()
    /// - REJECTED   -> ()
    #[instrument(name = "Mutation::change_application_status", skip(self, ctx))]
    async fn change_application_status(
        &self,
        ctx: &Context<'_>,
        input: ChangeApplicationStatusInput,
    ) -> Result<ChangeApplicationStatusResult> {
        let scope = checks::is_event(ctx)?;
        checks::has_at_least_role(ctx, UserRole::Organizer)?;

        let db = ctx.data_unchecked::<PgPool>();
        let mut txn = db.begin().await?;

        let Some(mut application) = Application::find(&scope.event, input.id, &mut txn)
            .await
            .extend()?
        else {
            return Ok(UserError::new(&["id"], "application not found").into());
        };

        if matches!(
            (application.status, input.status),
            (_, ApplicationStatus::Pending)
                | (ApplicationStatus::Waitlisted, ApplicationStatus::Waitlisted)
                | (ApplicationStatus::Accepted, _)
                | (ApplicationStatus::Rejected, _)
        ) {
            return Ok(
                UserError::new(&["status"], "invalid status transition for application").into(),
            );
        }

        application
            .update()
            .status(input.status)
            .save(&mut txn)
            .await
            .extend()?;

        let email = Email::find(input.id, &mut txn)
            .await
            .extend()?
            .expect("email must exist");

        let mail = ctx.data_unchecked::<mail::Client>().clone();
        tokio::task::spawn(async move {
            if let Err(error) = mail
                .send_templated(input.status.to_str(), &email.address)
                .await
            {
                error!(%error, "failed to send email")
            }
        });

        txn.commit().await?;

        Ok(application.into())
    }
}

/// Input fields for updating an application
#[derive(Debug, InputObject)]
struct UpdateApplicationInput {
    /// The ID of the application/participant
    id: i32,

    /// Whether the application needs extra review
    flagged: Option<bool>,
    /// Additional organizer-only notes
    notes: Option<String>,
}

/// Input fields for changing an application's status
#[derive(Debug, InputObject)]
struct ChangeApplicationStatusInput {
    /// The ID of the application/participant
    id: i32,
    /// The new status for the application
    status: ApplicationStatus,
}
