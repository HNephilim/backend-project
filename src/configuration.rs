use secrecy::{ExposeSecret, Secret};


#[derive(serde::Deserialize)]
pub struct Settings{
    pub database: DatabaseSettings,
    pub app_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings{
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub db_name: String,
}

impl DatabaseSettings{
    pub fn connection_string(&self) -> Secret<String>{
        Secret::new(format!("postgres://{}:{}@{}:{}/{}",
        self.username, self.password.expose_secret(), self.host, self.port, self.db_name))
    }

    pub fn connection_without_db_string(&self) -> Secret<String>{
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password.expose_secret(), self.host, self.port
        ))
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError>{
    //Get a default, or a reader, config
    let mut settings = config::Config::default();

    //read and parse the configuration file "config"
    settings.merge(config::File::with_name("config")).unwrap();

    //convert settings in our setting
    settings.try_into()

}