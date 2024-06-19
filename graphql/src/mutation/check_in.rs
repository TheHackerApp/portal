use super::{results, UserError};
use async_graphql::{Context, Object, Result, ResultExt};
use chrono::{DateTime, Utc};
use context::{checks, UserRole};
use database::{Application, CheckIn, PgPool};
use tracing::instrument;

results! {
    CheckInResult {
        /// WHen the participant checked in
        at: DateTime<Utc>,
    }
}

#[derive(Default)]
pub(crate) struct Mutation;

#[Object(name = "CheckInMutation")]
impl Mutation {
    /// Check in a participant to the event
    #[instrument(name = "Mutation::check_in", skip_all)]
    async fn check_in(&self, ctx: &Context<'_>, id: Option<i32>) -> Result<CheckInResult> {
        let scope = checks::is_event(ctx)?;
        let user = checks::is_authenticated(ctx)?;

        let id = if let Some(id) = id {
            if user.id == id {
                checks::has_role(ctx, UserRole::Participant)?;
            } else {
                checks::has_at_least_role(ctx, UserRole::Organizer)?;
            }

            id
        } else {
            checks::has_role(ctx, UserRole::Participant)?;
            user.id
        };

        let db = ctx.data_unchecked::<PgPool>();
        let mut txn = db.begin().await?;

        if !Application::accepted_exists(&scope.event, id, &mut *txn)
            .await
            .extend()?
        {
            return Ok(UserError::new(
                &["id"],
                "only participants with accepted applications can check in",
            )
            .into());
        }

        let check_in = CheckIn::mark(&scope.event, id, &mut *txn).await.extend()?;

        txn.commit().await?;

        Ok(check_in.at.into())
    }
}
