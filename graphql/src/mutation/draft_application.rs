use super::{results, UserError};
use crate::errors::Forbidden;
use async_graphql::{Context, InputObject, Object, Result, ResultExt};
use chrono::NaiveDate;
use context::{checks, UserRole};
use database::{Application, DraftApplication, Education, Gender, PgPool, RaceEthnicity};
use tracing::instrument;

results! {
    SaveApplicationResult {
        /// The saved, but unsubmitted application
        draft_application: DraftApplication,
    }
}

#[derive(Default)]
pub(crate) struct Mutation;

#[Object(name = "DraftApplicationMutation")]
impl Mutation {
    /// Save updates to an in-progress application
    #[instrument(name = "Mutation::save_application", skip_all)]
    async fn save_application(
        &self,
        ctx: &Context<'_>,
        input: SaveApplicationInput,
    ) -> Result<SaveApplicationResult> {
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
                UserError::new(&["saveApplication"], "application already submitted").into(),
            );
        }

        let mut draft = DraftApplication::find(&scope.event, user.id, db)
            .await
            .extend()?
            .unwrap_or_else(|| DraftApplication::new(scope.event.clone(), user.id));

        draft.gender = input.gender;
        draft.race_ethnicity = input.race_ethnicity;
        draft.date_of_birth = input.date_of_birth;
        draft.education = input.education;
        draft.graduation_year = input.graduation_year;
        draft.major = input.major;
        draft.hackathons_attended = input.hackathons_attended;
        draft.links = input.links;
        draft.address_line1 = input.address_line1;
        draft.address_line2 = input.address_line2;
        draft.address_line3 = input.address_line3;
        draft.locality = input.locality;
        draft.administrative_area = input.administrative_area;
        draft.postal_code = input.postal_code;
        draft.country = input.country;
        draft.share_information = input.share_information;

        draft.save(db).await.extend()?;

        Ok(draft.into())
    }
}

/// Input fields for saving an in-progress application
#[derive(Debug, InputObject)]
struct SaveApplicationInput {
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
}
