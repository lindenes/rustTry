use deadpool_postgres::{Config, Pool, Manager, ManagerConfig};
use config::Config as Configurator;
use serde::Deserialize;
use dotenv::dotenv;
use std::env;
#[derive(Debug, Deserialize)]
pub struct DbConfig{
    pub db_name:String,
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
}
#[derive(Debug, Deserialize)]
pub struct HttpConfig{
    pub port:u16
}
#[derive(Debug, Deserialize)]
pub struct AppConfig{
    #[serde(rename = "database")]
    pub db_config: DbConfig,
    #[serde(rename = "http")]
    pub http_config: HttpConfig
}

impl AppConfig {
     fn getConfiguration() -> AppConfig {
        let dbSettings =
            Configurator::builder()
                .add_source(config::File::with_name("./resources/application"))
                .build()
                .unwrap()
                .try_deserialize::<AppConfig>();
        dbSettings.unwrap()
    }

    fn loadConfiguration(config:AppConfig) -> AppConfig {
        dotenv().ok();
        let s = format!("{} not set",config.db_config.db_name);
        AppConfig{
            db_config:
                DbConfig{
                    db_name: env::var(config.db_config.db_name).expect(format!("{} not set",config.db_config.db_name).as_str()),
                    user:env::var(config.db_config.user).expect(format!("{} not set", config.db_config.user).as_str()),
                    password:env::var(config.db_config.password).expect(format!("{} not set",config.db_config.password).as_str()),
                    host:env::var(config.db_config.host).expect(format!("{} not set",config.db_config.host).as_str()),
                    port:env::var(config.db_config.port).expect(format!("{} not set",config.db_config.port)),
                }
        }

    }
}