use crate::Result;
#[cfg(feature = "graphql")]
use async_graphql::SimpleObject;
use sqlx::{query, query_as};
use tracing::instrument;
use uuid::Uuid;

/// A verified school
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "graphql", derive(SimpleObject))]
pub struct School {
    /// The school's unique ID
    pub id: String,
    /// The school's official name
    pub name: String,
}

impl_queries! {
    for School;

    /// Check if a school exists
    #[instrument(name = "School::exists", skip(conn))]
    pub async fn exists(id: &'a Uuid; conn) -> Result<bool> {
        let mut conn = conn.acquire().await?;
        let result = query!("SELECT exists(SELECT 1 FROM schools WHERE id = $1)", id)
            .fetch_one(&mut *conn)
            .await?;

        Ok(result.exists.unwrap_or_default())
    }

    /// Fetch all the schools
    #[instrument(name = "School::all", skip_all)]
    pub async fn all(; conn) -> Result<Vec<School>> {
        let mut conn = conn.acquire().await?;
        let result = query_as!(School, "SELECT * FROM schools")
            .fetch_all(&mut *conn)
            .await?;

        Ok(result)
    }

    /// Find a school by its ID
    #[instrument(name = "School::find", skip(conn))]
    pub async fn find(id: &'a Uuid; conn) -> Result<Option<School>> {
        let mut conn = conn.acquire().await?;
        let result = query_as!(School, "SELECT * FROM schools WHERE id = $1", id)
            .fetch_optional(&mut *conn)
            .await?;

        Ok(result)
    }

    /// Delete a school
    #[instrument(name = "School::delete", skip(conn))]
    pub async fn delete(id: &'a Uuid; conn) -> Result<()> {
        let mut conn = conn.acquire().await?;
        query!("DELETE FROM schools WHERE id = $1", id)
            .execute(&mut *conn)
            .await?;

        Ok(())
    }
}
