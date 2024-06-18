use crate::Result;
#[cfg(feature = "graphql")]
use crate::{
    stubs::{Event, Participant},
    School,
};
#[cfg(feature = "graphql")]
use async_graphql::{ComplexObject, Context, Enum, ResultExt, SimpleObject};
use chrono::{DateTime, NaiveDate, Utc};
#[cfg(feature = "graphql")]
use context::{
    checks::{guard_where, has_at_least_role},
    UserRole,
};
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "graphql")]
use serde::Serialize;
use sqlx::{query, query_as, Acquire, QueryBuilder};
use std::future::Future;
use tracing::instrument;
use uuid::Uuid;

/// A person's gender
#[derive(Clone, Copy, Debug, Eq, PartialEq, sqlx::Type)]
#[cfg_attr(feature = "graphql", derive(Enum))]
#[sqlx(rename_all = "kebab-case", type_name = "gender")]
#[cfg_attr(feature = "graphql", derive(Serialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum Gender {
    Male,
    Female,
    NonBinary,
    Other,
}

/// A person's race/ethnicity
#[derive(Clone, Copy, Debug, Eq, PartialEq, sqlx::Type)]
#[cfg_attr(feature = "graphql", derive(Enum))]
#[sqlx(rename_all = "kebab-case", type_name = "race_ethnicity")]
#[cfg_attr(feature = "graphql", derive(Serialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum RaceEthnicity {
    AsianIndian,
    Black,
    Chinese,
    Filipino,
    Guamanian,
    Hispanic,
    Japanese,
    Korean,
    MiddleEastern,
    NativeAmerican,
    NativeHawaiian,
    Samoan,
    Vietnamese,
    White,
    OtherAsian,
    OtherPacificIslander,
    Other,
}

/// A person's level of education
#[derive(Clone, Copy, Debug, Eq, PartialEq, sqlx::Type)]
#[cfg_attr(feature = "graphql", derive(Enum))]
#[sqlx(rename_all = "kebab-case", type_name = "education")]
#[cfg_attr(feature = "graphql", derive(Serialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum Education {
    BelowSecondary,
    Secondary,
    UndergraduateTwoYear,
    UndergraduateThreeYearPlus,
    Graduate,
    Bootcamp,
    Vocational,
    Other,
    NonStudent,
}

/// Where a person found the event
#[derive(Clone, Copy, Debug, Eq, PartialEq, sqlx::Type)]
#[cfg_attr(feature = "graphql", derive(Enum))]
#[sqlx(rename_all = "kebab-case", type_name = "referrer")]
#[cfg_attr(feature = "graphql", derive(Serialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum Referrer {
    Search,
    Peer,
    SocialMedia,
    Blog,
    Advertisement,
    StudentOrganization,
    School,
    Other,
}

/// The status of an application
#[derive(Clone, Copy, Debug, Eq, PartialEq, sqlx::Type)]
#[cfg_attr(feature = "graphql", derive(Enum))]
#[sqlx(rename_all = "lowercase", type_name = "application_status")]
#[cfg_attr(feature = "graphql", derive(Serialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum ApplicationStatus {
    Pending,
    Waitlisted,
    Rejected,
    Accepted,
}

/// An application to an event
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "graphql", derive(SimpleObject))]
#[cfg_attr(feature = "graphql", graphql(complex))]
#[cfg_attr(feature = "graphql", derive(Serialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Application {
    /// The slug of the event the application is for
    #[cfg_attr(feature = "graphql", graphql(skip))]
    pub event: String,
    /// The ID of the participant that submitted the application
    #[cfg_attr(feature = "graphql", graphql(skip))]
    pub participant_id: i32,

    /// The participant's gender
    pub gender: Gender,
    /// The participant's race/ethnicity
    pub race_ethnicity: RaceEthnicity,
    /// Participant birthday
    pub date_of_birth: NaiveDate,
    /// How the participant found the event
    pub referrer: Option<Referrer>,

    /// The school the participant attends
    #[cfg_attr(feature = "graphql", graphql(skip))]
    pub school_id: Option<Uuid>,
    /// The highest level of education the participant has achieved/is working on
    pub education: Education,
    /// When the participant will graduate/graduated
    pub graduation_year: i32,
    /// What the participant is studying
    pub major: Option<String>,

    /// How many hackathons the participant has attended
    pub hackathons_attended: i32,
    /// The public VCS URL (i.e. GitHub, GitLab, BitBucket, etc.)
    #[cfg_attr(feature = "schema", schemars(url))]
    pub vcs_url: Option<String>,
    /// The URL to the participant's portfolio
    #[cfg_attr(feature = "schema", schemars(url))]
    pub portfolio_url: Option<String>,
    /// The URL to the participant's DevPost profile
    #[cfg_attr(feature = "schema", schemars(url))]
    pub devpost_url: Option<String>,

    /// The first line of the shipping address
    #[cfg_attr(feature = "graphql", graphql(skip))]
    pub address_line1: String,
    /// The second line of the shipping address
    #[cfg_attr(feature = "graphql", graphql(skip))]
    pub address_line2: Option<String>,
    /// The last line of the shipping address
    #[cfg_attr(feature = "graphql", graphql(skip))]
    pub address_line3: Option<String>,
    /// The city/town of the shipping address
    #[cfg_attr(feature = "graphql", graphql(skip))]
    pub locality: Option<String>,
    /// The state/province/region of the shipping address
    #[cfg_attr(feature = "graphql", graphql(skip))]
    pub administrative_area: Option<String>,
    /// The postal code of the shipping address
    #[cfg_attr(feature = "graphql", graphql(skip))]
    pub postal_code: String,
    /// The ISO code of the country the shipping address is located in
    #[cfg_attr(feature = "graphql", graphql(skip))]
    pub country: String,

    /// Whether the participant wishes to share information with sponsors
    pub share_information: bool,

    /// The application's acceptance status
    pub status: ApplicationStatus,
    /// Whether the application needs extra review
    #[cfg_attr(
        feature = "graphql",
        graphql(guard = "guard_where(has_at_least_role, UserRole::Organizer)")
    )]
    pub flagged: bool,
    /// Additional organizer-only notes
    #[cfg_attr(
        feature = "graphql",
        graphql(guard = "guard_where(has_at_least_role, UserRole::Organizer)")
    )]
    pub notes: String,

    /// When the application was submitted
    pub created_at: DateTime<Utc>,
    /// When the application was last modified
    pub updated_at: DateTime<Utc>,
}

#[cfg(feature = "graphql")]
#[ComplexObject]
impl Application {
    /// The event the application is for
    async fn event(&self) -> Event<'_> {
        Event { slug: &self.event }
    }

    /// The participant who submitted the application
    async fn participant(&self) -> Participant {
        Participant::new(self.participant_id, &self.event)
    }

    /// The applicant's shipping address
    async fn address(&self) -> Address<'_> {
        Address {
            line1: &self.address_line1,
            line2: self.address_line2.as_deref(),
            line3: self.address_line3.as_deref(),
            locality: self.locality.as_deref(),
            administrative_area: self.locality.as_deref(),
            postal_code: &self.postal_code,
            country: &self.country,
        }
    }

    /// The school the participant attends
    #[instrument(name = "Application::school", skip_all)]
    async fn school(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<School>> {
        match &self.school_id {
            Some(school_id) => {
                let db = ctx.data_unchecked::<sqlx::PgPool>();
                School::find(school_id, db).await.extend()
            }
            None => Ok(None),
        }
    }
}

impl_queries! {
    for Application;

    /// Check if an application exists
    #[instrument(name = "Application::exists", skip(conn))]
    pub async fn exists(event: &'a str, participant_id: i32; conn) -> Result<bool> {
        let mut conn = conn.acquire().await?;
        let result = query!(
            "SELECT exists(SELECT 1 FROM applications WHERE participant_id = $1 AND event = $2)",
            participant_id,
            event
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(result.exists.unwrap_or_default())
    }

    /// Check if an accepted application exists
    #[instrument(name = "Application::accepted_exists", skip(conn))]
    pub async fn accepted_exists(event: &'a str, participant_id: i32; conn) -> Result<bool> {
        let mut conn = conn.acquire().await?;
        let result = query!(
            r#"
            SELECT exists(
                SELECT 1 FROM applications
                WHERE status = 'accepted' AND participant_id = $1 AND event = $2
            )
            "#,
            participant_id,
            event
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(result.exists.unwrap_or_default())
    }

    /// Get all the submitted applications for an event
    #[instrument(name = "Application::all", skip(conn))]
    pub async fn all(event: &'a str; conn) -> Result<Vec<Application>> {
        let mut conn = conn.acquire().await?;
        let applications = query_as!(
            Application,
            r#"
            SELECT
                event, participant_id,
                gender as "gender: Gender", race_ethnicity as "race_ethnicity: RaceEthnicity",
                date_of_birth, referrer as "referrer: Referrer",
                school_id, education as "education: Education", graduation_year, major,
                hackathons_attended, vcs_url, portfolio_url, devpost_url,
                address_line1, address_line2, address_line3, locality, administrative_area,
                postal_code, country, share_information,
                status as "status: ApplicationStatus", flagged, notes,
                created_at, updated_at
            FROM applications
            WHERE event = $1
            "#,
            event
        )
        .fetch_all(&mut *conn)
        .await?;

        Ok(applications)
    }

    /// Get an application by its event and participant id
    #[instrument(name = "Application::find", skip(conn))]
    pub async fn find(event: &'a str, participant_id: i32; conn) -> Result<Option<Application>> {
        let mut conn = conn.acquire().await?;
        let application = query_as!(
            Application,
            r#"
            SELECT
                event, participant_id,
                gender as "gender: Gender", race_ethnicity as "race_ethnicity: RaceEthnicity",
                date_of_birth, referrer as "referrer: Referrer",
                school_id, education as "education: Education", graduation_year, major,
                hackathons_attended, vcs_url, portfolio_url, devpost_url,
                address_line1, address_line2, address_line3, locality, administrative_area,
                postal_code, country, share_information,
                status as "status: ApplicationStatus", flagged, notes,
                created_at, updated_at
            FROM applications
            WHERE participant_id = $1 AND event = $2
            "#,
            participant_id,
            event
        )
        .fetch_optional(&mut *conn)
        .await?;

        Ok(application)
    }

    /// Create a new application from a draft
    #[instrument(name = "Application::from_draft", skip(conn))]
    pub async fn from_draft(event: &'a str, participant_id: i32; conn) -> Result<Self> {
        let mut conn = conn.acquire().await?;
        let application = query_as!(
            Application,
            r#"
            INSERT INTO applications (
                event, participant_id,
                gender, race_ethnicity, date_of_birth,
                education, graduation_year, major,
                hackathons_attended,
                address_line1, address_line2, address_line3, locality, administrative_area,
                postal_code, country,
                share_information,
                created_at, updated_at,
                vcs_url, portfolio_url, devpost_url,
                referrer, school_id
            )
            SELECT * FROM draft_applications
            WHERE participant_id = $1 AND event = $2
            RETURNING
                event, participant_id,
                gender as "gender: Gender", race_ethnicity as "race_ethnicity: RaceEthnicity",
                date_of_birth, referrer as "referrer: Referrer",
                school_id, education as "education: Education", graduation_year, major,
                hackathons_attended, vcs_url, portfolio_url, devpost_url,
                address_line1, address_line2, address_line3, locality, administrative_area,
                postal_code, country, share_information,
                status as "status: ApplicationStatus", flagged, notes,
                created_at, updated_at
            "#,
            participant_id,
            event,
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(application)
    }

    /// Delete an application
    #[instrument(name = "Application::delete", skip(conn))]
    pub async fn delete(event: &'a str, participant_id: i32; conn) -> Result<()> {
        let mut conn = conn.acquire().await?;
        query!(
            "DELETE FROM applications WHERE participant_id = $1 AND event = $2",
            participant_id,
            event
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }
}

impl Application {
    /// Update the application's fields
    pub fn update(&mut self) -> ApplicationUpdater<'_> {
        ApplicationUpdater::new(self)
    }
}

/// A person's shipping address
#[cfg(feature = "graphql")]
#[derive(Clone, Debug, Eq, PartialEq, SimpleObject)]
pub struct Address<'a> {
    line1: &'a str,
    line2: Option<&'a str>,
    line3: Option<&'a str>,
    locality: Option<&'a str>,
    administrative_area: Option<&'a str>,
    postal_code: &'a str,
    country: &'a str,
}

/// Handles updating an application
pub struct ApplicationUpdater<'a> {
    application: &'a mut Application,
    status: Option<ApplicationStatus>,
    flagged: Option<bool>,
    notes: Option<String>,
}

impl<'m> ApplicationUpdater<'m> {
    fn new(application: &'m mut Application) -> ApplicationUpdater<'m> {
        Self {
            application,
            status: None,
            flagged: None,
            notes: None,
        }
    }

    /// Update the status
    pub fn status(mut self, status: ApplicationStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Directly set the status
    pub fn override_status(mut self, status: Option<ApplicationStatus>) -> Self {
        self.status = status;
        self
    }

    /// Update the flagged state
    pub fn flagged(mut self, flagged: bool) -> Self {
        self.flagged = Some(flagged);
        self
    }

    /// Directly set the flagged state
    pub fn override_flagged(mut self, flagged: Option<bool>) -> Self {
        self.flagged = flagged;
        self
    }

    /// Update the status
    pub fn notes(mut self, notes: String) -> Self {
        self.notes = Some(notes);
        self
    }

    /// Directly set the status
    pub fn override_notes(mut self, notes: Option<String>) -> Self {
        self.notes = notes;
        self
    }

    /// Perform the update
    #[instrument(
        name = "Application::update",
        skip_all,
        fields(
            self.participant_id = self.application.participant_id,
            self.event = self.application.event
        )
    )]
    #[allow(clippy::manual_async_fn)]
    pub fn save<'a, 'c, A>(self, db: A) -> impl Future<Output = Result<()>> + Send + 'a
    where
        'm: 'a,
        A: 'a + Acquire<'c, Database = sqlx::Postgres> + Send,
    {
        async move {
            if self.status.is_none() && self.flagged.is_none() && self.notes.is_none() {
                // nothing was changed
                return Ok(());
            }

            let mut builder = QueryBuilder::new("UPDATE applications SET ");
            let mut separated = builder.separated(", ");

            if let Some(status) = self.status {
                separated.push("status = ");
                separated.push_bind_unseparated(status);
            }

            if let Some(flagged) = self.flagged {
                separated.push("flagged = ");
                separated.push_bind_unseparated(flagged);
            }

            if let Some(notes) = &self.notes {
                separated.push("notes = ");
                separated.push_bind_unseparated(notes);
            }

            builder.push(" WHERE participant_id = ");
            builder.push_bind(self.application.participant_id);
            builder.push(" AND event = ");
            builder.push_bind(&self.application.event);

            let mut conn = db.acquire().await?;
            builder.build().execute(&mut *conn).await?;

            if let Some(status) = self.status {
                self.application.status = status;
            }

            if let Some(flagged) = self.flagged {
                self.application.flagged = flagged;
            }

            if let Some(notes) = self.notes {
                self.application.notes = notes;
            }

            Ok(())
        }
    }
}
