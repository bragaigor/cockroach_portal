use uuid::Uuid;

use sqlx::{pool::PoolConnection, postgres::PgPoolOptions, PgPool, Postgres};
use std::sync::Arc;
use std::time::Duration;
use serde::{Deserialize, Serialize};

pub type ATResult<T> = anyhow::Result<T, ATError>;

pub type DBConn = PoolConnection<Postgres>;

#[derive(Debug, thiserror::Error)]
pub enum ATError {
    /// Generic unhandled error for postgres errors
    #[error(transparent)]
    Provider(sqlx::Error),

    #[error("parsing error")]
    ParseError,
}

#[derive(Debug, Clone)]
pub struct DB {
    pub pool: Arc<PgPool>,
    pub is_test: bool,
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct TransformerInput {
    pub title: String,
    pub power: i32,
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Transformer {
    pub id: Uuid,
    pub title: String,
    pub power: i32,
}

impl DB {

    pub async fn new_cockroach_test() -> ATResult<Self> {
        let url = dotenv::var("ROACH_DATABASE_URL").expect("must set ROACH_DATABASE_URL");
        println!("\tAbout to create a pool for Cockroach DB!!!");

        // Create a pool of size 1 so that we always get the same connection and never release it
        let pool = PgPoolOptions::new()
            .min_connections(1)
            .max_connections(1)
            .connect_timeout(Duration::from_secs(90))
            .connect(&url)
            .await
            .unwrap();

        Ok(Self {
            pool: Arc::new(pool),
            is_test: true,
        })
    }

    /// Get a connection to the pool
    pub async fn conn(&self) -> ATResult<DBConn> {
        let conn = self.pool.acquire().await;
        if let Err(_) = conn {
            println!("Error trying to acquire DB connection");
            return Err(ATError::ParseError);
        }
        Ok(conn.unwrap())
    }
}

/// Adds one or more WS buckets with friction factor
pub async fn add_transformer(dbc: &mut DBConn, transformer_input: TransformerInput) -> ATResult<Uuid> {

    #[derive(sqlx::FromRow)]
    struct ReturningId {
        id: Uuid,
    }

        let result = match sqlx::query_as::<_, ReturningId>(
            r#"
            INSERT INTO transformer(
                title,
                power
            )
            VALUES(
                $1,
                $2
            )
            RETURNING
                id
            "#,
        )
        .bind(transformer_input.title)
        .bind(transformer_input.power)
        .fetch_one(&mut *dbc)
        .await {
            Ok(r) => r,
            Err(error) => {panic!("add_transformer ERROR. Could not insert to DB: {:?}", error);}
        };

    Ok(result.id)
}

/// Get all issue occurrences on hour basis interval
pub async fn get_transformers(
    dbc: &mut DBConn,
) -> ATResult<Vec<Transformer>> {
    let rows = match sqlx::query_as::<_, Transformer>(
        r#"
        SELECT id, title, CAST(power AS INT4) AS power
        FROM transformer
        ORDER BY title
        "#)
    .fetch_all(&mut *dbc)
    .await {
        Ok(r) => r,
        Err(error) => {panic!("get_transformers ERROR. Could not query transformers: {:?}", error);}
    };

    Ok(rows)
}