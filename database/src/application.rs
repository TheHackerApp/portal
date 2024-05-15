use crate::Result;
#[cfg(feature = "graphql")]
use async_graphql::{ComplexObject, Enum, SimpleObject};
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{query, query_as, Executor, QueryBuilder};
use tracing::instrument;

/// A person's gender
#[derive(Clone, Copy, Debug, Eq, PartialEq, sqlx::Type)]
#[cfg_attr(feature = "graphql", derive(Enum))]
#[sqlx(rename_all = "kebab-case", type_name = "gender")]
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

/// The status of an application
#[derive(Clone, Copy, Debug, Eq, PartialEq, sqlx::Type)]
#[cfg_attr(feature = "graphql", derive(Enum))]
#[sqlx(rename_all = "lowercase", type_name = "application_status")]
pub enum ApplicationStatus {
    Pending,
    Waitlisted,
    Rejected,
    Accepted,
}

/// An application to an event
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "graphql", derive(SimpleObject))]
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

    /// The highest level of education the participant has achieved/is working on
    pub education: Education,
    /// When the participant will graduate/graduated
    pub graduation_year: i32,
    /// What the participant is studying
    pub major: Option<String>,

    /// How many hackathons the participant has attended
    pub hackathons_attended: i32,
    /// Public links the participant wishes to share (i.e. portfolio, GitHub, etc.)
    pub links: Vec<String>,

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

    // TODO: restrict these fields to organizers
    /// The application's acceptance status
    pub status: ApplicationStatus,
    /// Whether the application needs extra review
    pub flagged: bool,
    /// Additional organizer-only notes
    pub notes: String,

    /// When the application was submitted
    pub created_at: DateTime<Utc>,
    /// When the application was last modified
    pub updated_at: DateTime<Utc>,
}

#[cfg(feature = "graphql")]
#[ComplexObject]
impl Application {
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
}

impl Application {
    /// Check if an application exists
    #[instrument(name = "Application::exists", skip(db))]
    pub async fn exists<'c, 'e, E>(event: &str, participant_id: i32, db: E) -> Result<bool>
    where
        'c: 'e,
        E: 'e + Executor<'c, Database = sqlx::Postgres>,
    {
        let result = query!(
            "SELECT exists(SELECT 1 FROM applications WHERE participant_id = $1 AND event = $2)",
            participant_id,
            event
        )
        .fetch_one(db)
        .await?;

        Ok(result.exists.unwrap_or_default())
    }

    /// Get an application by its event and participant id
    #[instrument(name = "Application::find", skip(db))]
    pub async fn find<'c, 'e, E>(
        event: &str,
        participant_id: i32,
        db: E,
    ) -> Result<Option<Application>>
    where
        'c: 'e,
        E: 'e + Executor<'c, Database = sqlx::Postgres>,
    {
        let application = query_as!(
            Application,
            r#"
            SELECT
                event, participant_id,
                gender as "gender: Gender", race_ethnicity as "race_ethnicity: RaceEthnicity",
                date_of_birth,
                education as "education: Education", graduation_year, major,
                hackathons_attended, links,
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
        .fetch_optional(db)
        .await?;

        Ok(application)
    }

    /// Create a new application from a draft
    #[instrument(name = "Application::from_draft", skip(db))]
    pub async fn from_draft<'c, 'e, E>(event: &str, participant_id: i32, db: E) -> Result<Self>
    where
        'c: 'e,
        E: 'e + Executor<'c, Database = sqlx::Postgres>,
    {
        let application = query_as!(
            Application,
            r#"
            INSERT INTO applications (
                event, participant_id,
                gender, race_ethnicity, date_of_birth,
                education, graduation_year, major,
                hackathons_attended, links,
                address_line1, address_line2, address_line3, locality, administrative_area,
                postal_code, country,
                share_information,
                created_at, updated_at
            )
            SELECT * FROM draft_applications
            WHERE participant_id = $1 AND event = $2
            RETURNING
                event, participant_id,
                gender as "gender: Gender", race_ethnicity as "race_ethnicity: RaceEthnicity",
                date_of_birth,
                education as "education: Education", graduation_year, major,
                hackathons_attended, links,
                address_line1, address_line2, address_line3, locality, administrative_area,
                postal_code, country, share_information,
                status as "status: ApplicationStatus", flagged, notes,
                created_at, updated_at
            "#,
            participant_id,
            event,
        )
        .fetch_one(db)
        .await?;

        Ok(application)
    }

    /// Update the application's fields
    pub fn update(&mut self) -> ApplicationUpdater<'_> {
        ApplicationUpdater::new(self)
    }

    /// Delete an application
    #[instrument(name = "Application::delete", skip(db))]
    pub async fn delete<'c, 'e, E>(event: &str, participant_id: i32, db: E) -> Result<()>
    where
        'c: 'e,
        E: 'e + Executor<'c, Database = sqlx::Postgres>,
    {
        query!(
            "DELETE FROM applications WHERE participant_id = $1 AND event = $2",
            participant_id,
            event
        )
        .execute(db)
        .await?;

        Ok(())
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

impl<'a> ApplicationUpdater<'a> {
    fn new(application: &'a mut Application) -> ApplicationUpdater<'a> {
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
    pub async fn save<'c, 'e, E>(self, db: E) -> Result<()>
    where
        'c: 'e,
        E: 'e + Executor<'c, Database = sqlx::Postgres>,
    {
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

        builder.build().execute(db).await?;

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
