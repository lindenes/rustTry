use deadpool_postgres::{Config, Pool, Manager, ManagerConfig};
use config::Config as Configurator;
use serde::Deserialize;
use dotenv::dotenv;
use std::env;
#[derive(Debug, Deserialize)]
struct DbConfigEnv{
    db_name:String,
    user: String,
    password: String,
    host: String,
    port: String,
}
#[derive(Debug, Deserialize)]
struct HttpConfigEnv{
    port:String
}
#[derive(Debug, Deserialize)]
struct AppConfigEnv{
    #[serde(rename = "database")]
    db_config: DbConfigEnv,
    #[serde(rename = "http")]
    http_config: HttpConfigEnv
}

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
    pub db_config: crate::configuration::DbConfig,
    #[serde(rename = "http")]
    pub http_config: crate::configuration::HttpConfig
}

impl AppConfig {
    pub fn load_configuration() -> AppConfig {
        let config =
            Configurator::builder()
                .add_source(config::File::with_name("./resources/application"))
                .build()
                .unwrap()
                .try_deserialize::<AppConfigEnv>()
                .unwrap();
        dotenv().ok();
        AppConfig{
            db_config:
                DbConfig{
                    db_name: env::var(&config.db_config.db_name).expect(format!("{} not set",&config.db_config.db_name).as_str()),
                    user:env::var(&config.db_config.user).expect(format!("{} not set", &config.db_config.user).as_str()),
                    password:env::var(&config.db_config.password).expect(format!("{} not set",&config.db_config.password).as_str()),
                    host:env::var(&config.db_config.host).expect(format!("{} not set",&config.db_config.host).as_str()),
                    port:env::var(&config.db_config.port).expect(format!("{} not set",&config.db_config.port).as_str()).parse().expect(format!("invalid convert to u16 {}", &config.db_config.port).as_str()),
                },
            http_config:
                HttpConfig{
                    port:env::var(&config.http_config.port).expect(format!("{} not set",&config.db_config.port).as_str()).parse().expect(format!("invalid convert to u16 {}", &config.db_config.port).as_str()),
                }
        }

    }
}