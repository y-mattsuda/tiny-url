use core::panic;
use std::sync::Arc;
use std::{env, format};

use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

#[derive(Clone)]
pub struct Db(pub(crate) Arc<Pool<MySql>>);

impl Db {
    pub async fn new() -> Self {
        let db_host = env_var("MYSQL_HOST", Some("127.0.0.1".to_string()));
        let db_port = env_var("MYSQL_PORT", Some("3306".to_string()));
        let db_user = env_var("MYSQL_USER", None);
        let db_pass = env_var("MYSQL_PASSWORD", None);
        let db_name = env_var("MYSQL_DATABASE", None);
        let pool = MySqlPoolOptions::new()
            .max_connections(8)
            .connect(
                format!(
                    "mysql://{user}:{pass}@{host}:{port}/{name}",
                    user = db_user,
                    pass = db_pass,
                    host = db_host,
                    port = db_port,
                    name = db_name
                )
                .as_str(),
            )
            .await
            .unwrap_or_else(|_| {
                panic!("Cannot connect to the database. Please check your configuration")
            });
        Db(Arc::new(pool))
    }
}

fn env_var(name: &str, def_var: Option<String>) -> String {
    let env_var = env::var(name);
    match def_var {
        Some(v) => env_var.unwrap_or(v),
        _ => env_var.unwrap_or_else(|_| panic!("{name} must be set")),
    }
}
