use crate::{Education, Gender, RaceEthnicity, Result};
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{query, query_as, Executor};
use std::fmt::Debug;
use tracing::instrument;

/// An in-progress application from a participant
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DraftApplication {
    /// The slug of the event the application is for
    pub event: String,
    /// The ID of the participant that submitted the application
    pub participant_id: i32,

    /// The participant's gender
    pub gender: Option<Gender>,
    /// The participant's race/ethnicity
    pub race_ethnicity: Option<RaceEthnicity>,
    /// Participant birthday
    pub date_of_birth: Option<NaiveDate>,

    /// The highest level of education the participant has achieved/is working on
    pub education: Option<Education>,
    /// When the participant will graduate/graduated
    pub graduation_year: Option<i32>,
    /// What the participant is studying
    pub major: Option<String>,

    /// How many hackathons the participant has attended
    pub hackathons_attended: Option<i32>,
    /// Public links the participant wishes to share (i.e. portfolio, GitHub, etc.)
    pub links: Option<Vec<String>>,

    /// The first line of the shipping address
    pub address_line1: Option<String>,
    /// The second line of the shipping address
    pub address_line2: Option<String>,
    /// The last line of the shipping address
    pub address_line3: Option<String>,
    /// The city/town of the shipping address
    pub locality: Option<String>,
    /// The state/province/region of the shipping address
    pub administrative_area: Option<String>,
    /// The postal code of the shipping address
    pub postal_code: Option<String>,
    /// The ISO code of the country the shipping address is located in
    pub country: Option<String>,

    /// Whether the participant wishes to share information with sponsors
    pub share_information: Option<bool>,

    /// When the application was submitted
    pub created_at: DateTime<Utc>,
    /// When the application was last modified
    pub updated_at: DateTime<Utc>,
}

impl DraftApplication {
    /// Create a new draft application
    pub fn new(event: String, participant_id: i32) -> Self {
        Self {
            event,
            participant_id,
            gender: None,
            race_ethnicity: None,
            date_of_birth: None,
            education: None,
            graduation_year: None,
            major: None,
            hackathons_attended: None,
            links: None,
            address_line1: None,
            address_line2: None,
            address_line3: None,
            locality: None,
            administrative_area: None,
            postal_code: None,
            country: None,
            share_information: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Check if a draft application exists
    #[instrument(name = "Application::exists", skip(db))]
    pub async fn exists<'c, 'e, E>(event: &str, participant_id: i32, db: E) -> Result<bool>
    where
        'c: 'e,
        E: 'e + Executor<'c, Database = sqlx::Postgres>,
    {
        let result = query!(
            "SELECT exists(SELECT 1 FROM draft_applications WHERE participant_id = $1 AND event = $2)",
            participant_id,
            event
        )
        .fetch_one(db)
        .await?;

        Ok(result.exists.unwrap_or_default())
    }

    /// Get a draft application by the event and participant ID
    #[instrument(name = "DraftApplication::find")]
    pub async fn find<'c, 'e, E>(
        event: &str,
        participant_id: i32,
        db: E,
    ) -> Result<Option<DraftApplication>>
    where
        'c: 'e,
        E: 'e + Executor<'c, Database = sqlx::Postgres>,
    {
        let draft = query_as!(
            DraftApplication,
            r#"
            SELECT
                event, participant_id,
                gender as "gender: Gender", race_ethnicity as "race_ethnicity: RaceEthnicity",
                date_of_birth, education as "education: Education", graduation_year, major,
                hackathons_attended, links,
                address_line1, address_line2, address_line3, locality,
                administrative_area, postal_code, country,
                share_information,
                created_at, updated_at
            FROM draft_applications
            WHERE participant_id = $1 AND event = $2
            "#,
            participant_id,
            event
        )
        .fetch_optional(db)
        .await?;

        Ok(draft)
    }

    /// Save the draft application
    #[instrument(
        name = "DraftApplication::save",
        skip_all,
        fields(event = self.event, participant_id = self.participant_id)
    )]
    pub async fn save<'c, 'e, E>(&self, db: E) -> Result<()>
    where
        'c: 'e,
        E: 'e + Executor<'c, Database = sqlx::Postgres>,
    {
        query!(
            r#"
            INSERT INTO draft_applications (
                event, participant_id,
                gender, race_ethnicity, date_of_birth,
                education, graduation_year, major,
                hackathons_attended, links,
                address_line1, address_line2, address_line3, locality, administrative_area,
                postal_code, country,
                share_information
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
            ON CONFLICT (event, participant_id)
            DO UPDATE
                SET
                    gender = excluded.gender,
                    race_ethnicity = excluded.race_ethnicity,
                    date_of_birth = excluded.date_of_birth,
                    education = excluded.education,
                    graduation_year = excluded.graduation_year,
                    major = excluded.major,
                    hackathons_attended = excluded.hackathons_attended,
                    links = excluded.links,
                    address_line1 = excluded.address_line1,
                    address_line2 = excluded.address_line2,
                    address_line3 = excluded.address_line3,
                    locality = excluded.locality,
                    administrative_area = excluded.administrative_area,
                    postal_code = excluded.postal_code,
                    country = excluded.country,
                    share_information = excluded.share_information
                WHERE
                    draft_applications.participant_id = excluded.participant_id
                    AND draft_applications.event = excluded.event
            "#,
            self.event,
            self.participant_id,
            self.gender as _,
            self.race_ethnicity as _,
            self.date_of_birth,
            self.education as _,
            self.graduation_year,
            self.major,
            self.hackathons_attended,
            self.links.as_deref(),
            self.address_line1,
            self.address_line2,
            self.address_line3,
            self.locality,
            self.administrative_area,
            self.postal_code,
            self.country,
            self.share_information,
        )
        .execute(db)
        .await?;

        Ok(())
    }

    /// Delete a draft application
    #[instrument(name = "DraftApplication::delete", skip(db))]
    pub async fn delete<'c, 'e, E>(event: &str, participant_id: i32, db: E) -> Result<()>
    where
        'c: 'e,
        E: 'e + Executor<'c, Database = sqlx::Postgres>,
    {
        query!(
            "DELETE FROM draft_applications WHERE participant_id = $1 AND event = $2",
            participant_id,
            event
        )
        .execute(db)
        .await?;

        Ok(())
    }
}