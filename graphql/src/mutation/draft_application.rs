use super::{results, UserError};
use crate::errors::Forbidden;
use async_graphql::{Context, InputObject, MaybeUndefined, Object, Result, ResultExt};
use chrono::NaiveDate;
use context::{checks, UserRole};
use database::{Application, DraftApplication, Education, Gender, PgPool, RaceEthnicity, Referrer};
use tracing::instrument;

results! {
    SaveApplicationResult {
        /// The saved, but unsubmitted application
        draft_application: DraftApplication,
    }
}

macro_rules! set_option {
    ($source:expr => $destination:expr) => {
        match $source {
            MaybeUndefined::Value(value) => $destination = Some(value),
            MaybeUndefined::Null => $destination = None,
            MaybeUndefined::Undefined => {}
        }
    };
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
        let mut txn = db.begin().await?;

        if Application::exists(&scope.event, user.id, &mut txn)
            .await
            .extend()?
        {
            return Ok(
                UserError::new(&["saveApplication"], "application already submitted").into(),
            );
        }

        let mut draft = DraftApplication::find(&scope.event, user.id, &mut txn)
            .await
            .extend()?
            .unwrap_or_else(|| DraftApplication::new(scope.event.clone(), user.id));

        set_option!(input.gender => draft.gender);
        set_option!(input.race_ethnicity => draft.race_ethnicity);
        set_option!(input.date_of_birth => draft.date_of_birth);
        set_option!(input.referrer => draft.referrer);
        set_option!(input.education => draft.education);
        set_option!(input.graduation_year => draft.graduation_year);
        set_option!(input.major => draft.major);
        set_option!(input.hackathons_attended => draft.hackathons_attended);
        set_option!(input.vcs_url => draft.vcs_url);
        set_option!(input.portfolio_url => draft.portfolio_url);
        set_option!(input.devpost_url => draft.devpost_url);
        set_option!(input.address_line1 => draft.address_line1);
        set_option!(input.address_line2 => draft.address_line2);
        set_option!(input.address_line3 => draft.address_line3);
        set_option!(input.locality => draft.locality);
        set_option!(input.administrative_area => draft.administrative_area);
        set_option!(input.postal_code => draft.postal_code);
        set_option!(input.country => draft.country);
        if let Some(share_information) = input.share_information {
            draft.share_information = share_information;
        }

        draft.save(&mut txn).await.extend()?;

        txn.commit().await?;

        Ok(draft.into())
    }
}

/// Input fields for saving an in-progress application
#[derive(Debug, InputObject)]
struct SaveApplicationInput {
    /// The participant's gender
    pub gender: MaybeUndefined<Gender>,
    /// The participant's race/ethnicity
    pub race_ethnicity: MaybeUndefined<RaceEthnicity>,
    /// Participant birthday
    pub date_of_birth: MaybeUndefined<NaiveDate>,
    /// How the participant found the event
    pub referrer: MaybeUndefined<Referrer>,

    /// The highest level of education the participant has achieved/is working on
    pub education: MaybeUndefined<Education>,
    /// When the participant will graduate/graduated
    pub graduation_year: MaybeUndefined<i32>,
    /// What the participant is studying
    pub major: MaybeUndefined<String>,

    /// How many hackathons the participant has attended
    pub hackathons_attended: MaybeUndefined<i32>,
    /// The public VCS URL (i.e. GitHub, GitLab, BitBucket, etc.)
    pub vcs_url: MaybeUndefined<String>,
    /// The URL to the participant's portfolio
    pub portfolio_url: MaybeUndefined<String>,
    /// The URL to the participant's DevPost profile
    pub devpost_url: MaybeUndefined<String>,

    /// The first line of the shipping address
    pub address_line1: MaybeUndefined<String>,
    /// The second line of the shipping address
    pub address_line2: MaybeUndefined<String>,
    /// The last line of the shipping address
    pub address_line3: MaybeUndefined<String>,
    /// The city/town of the shipping address
    pub locality: MaybeUndefined<String>,
    /// The state/province/region of the shipping address
    pub administrative_area: MaybeUndefined<String>,
    /// The postal code of the shipping address
    pub postal_code: MaybeUndefined<String>,
    /// The ISO code of the country the shipping address is located in
    pub country: MaybeUndefined<String>,

    /// Whether the participant wishes to share information with sponsors
    pub share_information: Option<bool>,
}
