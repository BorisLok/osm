use std::sync::Arc;

use sqlx::{Pool, Postgres};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::core::environment::Environment;

#[derive(Debug, Clone)]
pub struct PostgresConnection {
    pub pool: Arc<Pool<Postgres>>,
}

impl PostgresConnection {
    pub async fn new(env: &Environment) -> anyhow::Result<Self> {
        let postgres_options = PgConnectOptions::new()
            .database(&env.postgres_database)
            .host(&env.postgres_host)
            .username(&env.postgres_user)
            .password(&env.postgres_password)
            .port(env.postgres_port);

        let connection_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect_with(postgres_options)
            .await?;

        Ok(Self {
            pool: Arc::new(connection_pool)
        })
    }
}