use chrono::{DateTime, Days, NaiveDate, Utc};
use database::{Application, ApplicationStatus, Education, Gender, RaceEthnicity};
use graphql::Payload;
use uuid::Uuid;

const SCHOOL_ID: Uuid = Uuid::from_fields(
    0x592dc687,
    0x6c47,
    0x40b1,
    &[0x94, 0x16, 0x4e, 0x23, 0xd8, 0x22, 0x8d, 0x5c],
);

const DATE_TIME: DateTime<Utc> = DateTime::from_timestamp_nanos(1716431863_000_000_000);

pub fn payload<'p, T>(kind: &'static str, object: &'p T) -> Payload<'p, T> {
    Payload {
        type_: kind,
        for_: "wafflehacks-2024",
        object,
        at: DATE_TIME,
    }
}

pub fn application() -> Application {
    Application {
        event: String::from("wafflehacks-2024"),
        participant_id: 3,
        gender: Gender::NonBinary,
        race_ethnicity: RaceEthnicity::Other,
        date_of_birth: NaiveDate::from_ymd_opt(2000, 10, 15).unwrap(),
        referrer: None,
        school_id: Some(SCHOOL_ID),
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
        created_at: DATE_TIME.checked_sub_days(Days::new(7)).unwrap(),
        updated_at: DATE_TIME,
    }
}
