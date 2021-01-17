use serde::Deserialize;
use config::ConfigError;

#[derive(Deserialize)]
pub struct ServerConfig{
    pub host: String,
    pub port: i32

}
#[derive(Deserialize)]
pub struct Config{
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config
}

impl Config {
    pub fn from_env() -> Result<Self,ConfigError> {
        let mut config = config::Config::new();
        config.merge(config::Environment::new())?;
        config.try_into()
    }
}