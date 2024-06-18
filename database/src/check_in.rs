use crate::Result;
use chrono::{DateTime, Utc};
use sqlx::{query, query_as};
use tracing::instrument;

/// An entry denoting a participant has been checked in
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckIn {
    /// The event's unique ID
    pub event: String,
    /// THe participant's unique ID
    pub participant_id: i32,
    /// WHen the participant checked in
    pub at: DateTime<Utc>,
}

impl_queries! {
    for CheckIn;

    /// Check if a participant checked in to an event
    #[instrument(name = "CheckIn::exists", skip(conn))]
    pub async fn exists(event: &'a str, participant_id: i32; conn) -> Result<bool> {
        let mut conn = conn.acquire().await?;
        let result = query!(
            "SELECT exists(SELECT 1 FROM check_ins WHERE participant_id = $1 AND event = $2)",
            participant_id,
            event,
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(result.exists.unwrap_or_default())
    }

    /// Mark a participant as being checked in
    #[instrument(name = "CheckIn::mark", skip(conn))]
    pub async fn mark(event: &'a str, participant_id: i32; conn) -> Result<CheckIn> {
        let mut conn = conn.acquire().await?;
        let check_in = query_as!(
            CheckIn,
            r#"
            INSERT INTO check_ins (event, participant_id)
            VALUES ($1, $2)
            ON CONFLICT (event, participant_id)
                DO UPDATE SET
                    event = excluded.event,
                    participant_id = excluded.participant_id
            RETURNING *
            "#,
            event,
            participant_id,
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(check_in)
    }

    /// Delete a check in
    #[instrument(name = "CheckIn::delete", skip(conn))]
    pub async fn delete(event: &'a str, participant_id: i32; conn) -> Result<()> {
        let mut conn = conn.acquire().await?;
        query!(
            "DELETE FROM check_ins WHERE participant_id = $1 AND event = $2",
            participant_id,
            event
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }
}
