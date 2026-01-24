use std::env;

pub struct Config {
    pub rpc_url: String,
    pub database_url: String,
    pub governor: String,
    pub timelock: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        Self {
            rpc_url: env::var("RPC_URL").unwrap(),
            database_url: env::var("DATABASE_URL").unwrap(),
            governor: env::var("GOVERNOR_ADDRESS").unwrap(),
            timelock: env::var("TIMELOCK_ADDRESS").unwrap(),
        }
    }
}
