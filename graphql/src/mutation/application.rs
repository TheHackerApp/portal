use super::{results, UserError};
use crate::errors::Forbidden;
use async_graphql::{Context, ErrorExtensions, Object, Result, ResultExt};
use context::{checks, UserRole};
use database::{Application, DraftApplication, PgPool};
use tracing::instrument;

results! {
    SubmitApplicationResult {
        /// The submitted application
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

        if user.role != Some(UserRole::Participant) {
            return Err(Forbidden.into());
        }

        let db = ctx.data_unchecked::<PgPool>();
        if Application::exists(&scope.event, user.id, db)
            .await
            .extend()?
        {
            return Ok(
                UserError::new(&["submitApplication"], "application already submitted").into(),
            );
        }

        let application = match Application::from_draft(&scope.event, user.id, db).await {
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

        DraftApplication::delete(&scope.event, user.id, db)
            .await
            .extend()?;

        Ok(application.into())
    }
}
