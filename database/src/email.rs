use super::Result;
use sqlx::query_as;
use tracing::instrument;

/// A temporary table for associating a participant with their email address
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Email {
    /// Who the email is associated with
    pub participant_id: i32,
    /// The email address to use
    pub address: String,
}

impl_queries! {
    for Email;

    /// Find an email for a participant
    #[instrument(name = "Email::find", skip(conn))]
    pub async fn find(id: i32; conn) -> Result<Option<Email>> {
        let mut conn = conn.acquire().await?;
        let email = query_as!(
            Email,
            "SELECT * FROM emails WHERE participant_id = $1",
            id
        )
        .fetch_optional(&mut *conn)
        .await?;

        Ok(email)
    }

    /// Associate an email with a participant
    #[instrument(name = "Email::upsert", skip(conn))]
    pub async fn upsert(id: i32, address: &'a str; conn) -> Result<Email> {
        let mut conn = conn.acquire().await?;
        let email = query_as!(
            Email,
            r#"
            INSERT INTO emails VALUES ($1, $2)
            ON CONFLICT (participant_id) DO UPDATE SET address = excluded.address
            RETURNING *
            "#,
            id,
            address
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(email)
    }
}
