use deadpool_postgres::{Config, Pool, Runtime::Tokio1};
use dotenv::dotenv;
use tokio_postgres::NoTls;

pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

pub struct AppConfig {
    pub server: ServerConfig,
    pub db: Pool,
}

impl AppConfig {
    pub fn from_env() -> AppConfig {
        // ensure .env parsing works
        match dotenv().ok() {
            None => {
                panic!("Could not open this file");
            }
            _ => {}
        }

        // make db config
        let pg_config = Config::new();

        AppConfig {
            server: ServerConfig {
                host: get_from_env("SERVER.HOST"),
                port: get_from_env("SERVER.PORT").parse::<u16>().unwrap_or(8000),
            },
            db: pg_config.create_pool(Some(Tokio1), NoTls).unwrap(),
        }
    }
}

fn get_from_env(key: &str) -> String {
    dotenv::var(key).unwrap()
}
