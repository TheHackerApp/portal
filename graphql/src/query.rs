use crate::errors::Forbidden;
use async_graphql::{Context, Object, Result, ResultExt};
use context::{checks, UserRole};
use database::{Application, DraftApplication, PgPool};
use svix::api::{AppPortalAccessIn, Svix};
use tracing::instrument;

/// When the webhook dashboard should expire (1 day in seconds)
const WEBHOOK_DASHBOARD_EXPIRY: i32 = 60 * 60 * 24;

pub struct Query;

#[Object]
impl Query {
    /// Get the URL for the webhook portal
    #[graphql(shareable)]
    #[instrument(name = "Query::webhook_dashboard_url", skip_all)]
    async fn webhook_dashboard_url(&self, ctx: &Context<'_>) -> Result<String> {
        let event = checks::is_event(ctx)?;
        let role = checks::has_at_least_role(ctx, UserRole::Organizer)?;

        let options = AppPortalAccessIn {
            expiry: Some(WEBHOOK_DASHBOARD_EXPIRY),
            read_only: Some(role == UserRole::Organizer),
            ..AppPortalAccessIn::default()
        };

        let svix = ctx.data_unchecked::<Svix>();
        let dashboard = svix
            .authentication()
            .app_portal_access(event.event.clone(), options, None)
            .await?;

        Ok(dashboard.url)
    }

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
        let scope = checks::is_event(ctx)?;
        checks::has_at_least_role(ctx, UserRole::Organizer)?;

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
