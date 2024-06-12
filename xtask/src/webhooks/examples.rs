use chrono::{Days, NaiveDate, Utc};
use database::{Application, ApplicationStatus, Education, Gender, RaceEthnicity};
use uuid::Uuid;

pub fn application() -> Application {
    Application {
        event: String::from("wafflehacks-2024"),
        participant_id: 3,
        gender: Gender::NonBinary,
        race_ethnicity: RaceEthnicity::Other,
        date_of_birth: NaiveDate::from_ymd_opt(2000, 10, 15).unwrap(),
        referrer: None,
        school_id: Some(Uuid::new_v4()),
        education: Education::UndergraduateThreeYearPlus,
        graduation_year: 2026,
        major: Some(String::from("Computer Science")),
        hackathons_attended: 2,
        vcs_url: Some(String::from("https://github.com/akrantz01")),
        portfolio_url: None,
        devpost_url: None,
        address_line1: String::from("999 Canada Place"),
        address_line2: None,
        address_line3: None,
        locality: Some(String::from("Vancouver")),
        administrative_area: Some(String::from("British Columbia")),
        postal_code: String::from("V6C 3T4"),
        country: String::from("Canada"),
        share_information: true,
        status: ApplicationStatus::Pending,
        flagged: false,
        notes: String::new(),
        created_at: Utc::now().checked_sub_days(Days::new(7)).unwrap(),
        updated_at: Utc::now(),
    }
}
