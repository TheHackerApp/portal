use crate::errors::Forbidden;
use async_graphql::{Context, Object, Result, ResultExt};
use context::{checks, UserRole};
use database::{Application, DraftApplication, PgPool};
use tracing::instrument;

pub struct Query;

#[Object]
impl Query {
    /// Get a submitted application
    #[instrument(name = "Query::application", skip(self, ctx))]
    async fn application(&self, ctx: &Context<'_>, id: Option<i32>) -> Result<Option<Application>> {
        let user = checks::is_authenticated(ctx)?;
        let scope = checks::is_event(ctx)?;

        // must be part of event
        if user.role.is_none() {
            return Err(Forbidden.into());
        }

        let id = if let Some(id) = id {
            if user.role == Some(UserRole::Participant) && id != user.id {
                return Err(Forbidden.into());
            }

            id
        } else {
            user.id
        };

        let db = ctx.data_unchecked::<PgPool>();
        let application = Application::find(&scope.event, id, db).await.extend()?;

        Ok(application)
    }

    /// Get all submitted applications
    #[instrument(name = "Query::applications", skip_all)]
    async fn applications(&self, ctx: &Context<'_>) -> Result<Vec<Application>> {
        let user = checks::is_authenticated(ctx)?;
        let scope = checks::is_event(ctx)?;

        let is_organizer = match user.role {
            Some(role) => role != UserRole::Participant,
            None => false,
        };
        if !is_organizer {
            return Err(Forbidden.into());
        }

        let db = ctx.data_unchecked::<PgPool>();
        let applications = Application::all(&scope.event, db).await.extend()?;

        Ok(applications)
    }

    /// Get an in-progress application
    #[instrument(name = "Query::draft_application", skip(self, ctx))]
    async fn draft_application(
        &self,
        ctx: &Context<'_>,
        id: Option<i32>,
    ) -> Result<Option<DraftApplication>> {
        let user = checks::is_authenticated(ctx)?;
        let scope = checks::is_event(ctx)?;

        // must be part of event
        if user.role.is_none() {
            return Err(Forbidden.into());
        }

        let id = if let Some(id) = id {
            if user.role == Some(UserRole::Participant) && id != user.id {
                return Err(Forbidden.into());
            }

            id
        } else {
            user.id
        };

        let db = ctx.data_unchecked::<PgPool>();
        let draft = DraftApplication::find(&scope.event, id, db)
            .await
            .extend()?;

        Ok(draft)
    }
}
