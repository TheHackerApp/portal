#[cfg(feature = "graphql")]
use crate::{
    stubs::{Event, Participant},
    School,
};
use crate::{Education, Gender, RaceEthnicity, Referrer, Result};
#[cfg(feature = "graphql")]
use async_graphql::{ComplexObject, Context, ResultExt, SimpleObject};
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{query, query_as, Acquire};
use std::{fmt::Debug, future::Future};
use tracing::instrument;
use uuid::Uuid;

/// An in-progress application from a participant
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "graphql", derive(SimpleObject))]
#[cfg_attr(feature = "graphql", graphql(complex))]
pub struct DraftApplication {
    /// The slug of the event the application is for
    #[cfg_attr(feature = "graphql", graphql(skip))]
    pub event: String,
    /// The ID of the participant that submitted the application
    #[cfg_attr(feature = "graphql", graphql(skip))]
    pub participant_id: i32,

    /// The participant's gender
    pub gender: Option<Gender>,
    /// The participant's race/ethnicity
    pub race_ethnicity: Option<RaceEthnicity>,
    /// Participant birthday
    pub date_of_birth: Option<NaiveDate>,
    /// How the participant found the event
    pub referrer: Option<Referrer>,

    /// The school the participant attends
    #[cfg_attr(feature = "graphql", graphql(skip))]
    pub school_id: Option<Uuid>,
    /// The highest level of education the participant has achieved/is working on
    pub education: Option<Education>,
    /// When the participant will graduate/graduated
    pub graduation_year: Option<i32>,
    /// What the participant is studying
    pub major: Option<String>,

    /// How many hackathons the participant has attended
    pub hackathons_attended: Option<i32>,
    /// The public VCS URL (i.e. GitHub, GitLab, BitBucket, etc.)
    pub vcs_url: Option<String>,
    /// The URL to the participant's portfolio
    pub portfolio_url: Option<String>,
    /// The URL to the participant's DevPost profile
    pub devpost_url: Option<String>,

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
    pub share_information: bool,

    /// When the application was submitted
    pub created_at: DateTime<Utc>,
    /// When the application was last modified
    pub updated_at: DateTime<Utc>,
}

#[cfg(feature = "graphql")]
#[ComplexObject]
impl DraftApplication {
    /// The event the application is for
    async fn event(&self) -> Event<'_> {
        Event { slug: &self.event }
    }

    /// The participant who submitted the application
    async fn participant(&self) -> Participant {
        Participant::new(self.participant_id, &self.event)
    }

    /// The school the participant attends
    #[instrument(name = "DraftApplication::school", skip_all)]
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

impl DraftApplication {
    /// Create a new draft application
    pub fn new(event: String, participant_id: i32) -> Self {
        Self {
            event,
            participant_id,
            gender: None,
            race_ethnicity: None,
            date_of_birth: None,
            referrer: None,
            school_id: None,
            education: None,
            graduation_year: None,
            major: None,
            hackathons_attended: None,
            vcs_url: None,
            portfolio_url: None,
            devpost_url: None,
            address_line1: None,
            address_line2: None,
            address_line3: None,
            locality: None,
            administrative_area: None,
            postal_code: None,
            country: None,
            share_information: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Save the draft application
    #[instrument(
        name = "DraftApplication::save",
        skip_all,
        fields(event = self.event, participant_id = self.participant_id)
    )]
    #[allow(clippy::manual_async_fn)]
    pub fn save<'a, 'c, A>(&'a self, db: A) -> impl Future<Output = Result<()>> + Send + 'a
    where
        A: 'a + Acquire<'c, Database = sqlx::Postgres> + Send,
    {
        async move {
            let mut conn = db.acquire().await?;
            query!(
                r#"
                INSERT INTO draft_applications (
                    event, participant_id,
                    gender, race_ethnicity, date_of_birth, referrer,
                    school_id, education, graduation_year, major,
                    hackathons_attended, vcs_url, portfolio_url, devpost_url,
                    address_line1, address_line2, address_line3, locality, administrative_area,
                    postal_code, country,
                    share_information
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
                ON CONFLICT (event, participant_id)
                DO UPDATE
                    SET
                        gender = excluded.gender,
                        race_ethnicity = excluded.race_ethnicity,
                        date_of_birth = excluded.date_of_birth,
                        referrer = excluded.referrer,
                        school_id = excluded.school_id,
                        education = excluded.education,
                        graduation_year = excluded.graduation_year,
                        major = excluded.major,
                        hackathons_attended = excluded.hackathons_attended,
                        vcs_url = excluded.vcs_url,
                        portfolio_url = excluded.portfolio_url,
                        devpost_url = excluded.devpost_url,
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
                self.referrer as _,
                self.school_id,
                self.education as _,
                self.graduation_year,
                self.major,
                self.hackathons_attended,
                self.vcs_url,
                self.portfolio_url,
                self.devpost_url,
                self.address_line1,
                self.address_line2,
                self.address_line3,
                self.locality,
                self.administrative_area,
                self.postal_code,
                self.country,
                self.share_information,
            )
            .execute(&mut *conn)
            .await?;

            Ok(())
        }
    }
}

impl_queries! {
    for DraftApplication;

    /// Check if a draft application exists
    #[instrument(name = "Application::exists", skip(db))]
    pub async fn exists(event: &'a str, participant_id: i32; db) -> Result<bool> {
        let mut conn = db.acquire().await?;
        let result = query!(
            "SELECT exists(SELECT 1 FROM draft_applications WHERE participant_id = $1 AND event = $2)",
            participant_id,
            event
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(result.exists.unwrap_or_default())
    }

    /// Get a draft application by the event and participant ID
    #[instrument(name = "DraftApplication::find", skip(db))]
    pub async fn find(event: &'a str, participant_id: i32; db) -> Result<Option<DraftApplication>> {
        let mut conn = db.acquire().await?;
        let draft = query_as!(
            DraftApplication,
            r#"
            SELECT
                event, participant_id,
                gender as "gender: Gender", race_ethnicity as "race_ethnicity: RaceEthnicity",
                date_of_birth, referrer as "referrer: Referrer",
                school_id, education as "education: Education", graduation_year, major,
                hackathons_attended, vcs_url, portfolio_url, devpost_url,
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
        .fetch_optional(&mut *conn)
        .await?;

        Ok(draft)
    }

    /// Delete a draft application
    #[instrument(name = "DraftApplication::delete", skip(db))]
    pub async fn delete(event: &'a str, participant_id: i32; db) -> Result<()>
    {
        let mut conn = db.acquire().await?;
        query!(
            "DELETE FROM draft_applications WHERE participant_id = $1 AND event = $2",
            participant_id,
            event
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }
}
