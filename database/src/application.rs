use chrono::{DateTime, NaiveDate, Utc};

/// A person's gender
#[derive(Clone, Copy, Debug, Eq, PartialEq, sqlx::Type)]
#[sqlx(rename_all = "kebab-case", type_name = "gender")]
pub enum Gender {
    Male,
    Female,
    NonBinary,
    Other,
}

/// A person's race/ethnicity
#[derive(Clone, Copy, Debug, Eq, PartialEq, sqlx::Type)]
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
#[sqlx(rename_all = "lowercase", type_name = "application_status")]
pub enum ApplicationStatus {
    Pending,
    Waitlisted,
    Rejected,
    Accepted,
}

/// An application to an event
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Application {
    /// The slug of the event the application is for
    pub event: String,
    /// The ID of the participant that submitted the application
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
    pub address_line1: String,
    /// The second line of the shipping address
    pub address_line2: Option<String>,
    /// The last line of the shipping address
    pub address_line3: Option<String>,
    /// The city/town of the shipping address
    pub locality: Option<String>,
    /// The state/province/region of the shipping address
    pub administrative_area: Option<String>,
    /// The postal code of the shipping address
    pub postal_code: String,
    /// The ISO code of the country the shipping address is located in
    pub country: String,

    /// Whether the participant wishes to share information with sponsors
    pub share_information: bool,

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
