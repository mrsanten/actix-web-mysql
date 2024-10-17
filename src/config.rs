use serde::Deserialize;
use std::fs;
use std::error::Error;

#[derive(Deserialize)]
struct AppConfig {
    url: String,
    port: u16,
}

#[derive(Deserialize)]
struct DaoConfig {
    url: String,
    port: u16,
    user: String,
    password: String,
    database: String,
}

#[derive(Deserialize)]
pub struct Config {
    app: AppConfig,
    dao: DaoConfig,
}

impl Config {
    // Loads configuration from a file and returns a Result
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        // Read the file's contents
        let config = fs::read_to_string(path)?;
        // Deserialize the configuration
        let parsed_config: Config = serde_json::from_str(&config)?;
        Ok(parsed_config)
    }

    // Returns the application's URL with the port
    pub fn get_app_url(&self) -> String {
        format!("{}:{}", self.app.url, self.app.port)
    }

    // Returns the database URL formatted for MySQL connection
    pub fn get_database_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.dao.user, self.dao.password, self.dao.url, self.dao.port, self.dao.database
        )
    }
}