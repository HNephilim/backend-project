
#[derive(serde::Deserialize)]
pub struct Settings{
    pub database: DatabaseSettings,
    pub app_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings{
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub db_name: String,
}

impl DatabaseSettings{
    pub fn connection_string(&self) -> String{
        format!("postgres://{}:{}@{}:{}/{}",
        self.username, self.password, self.host, self.port, self.db_name)
    }

    pub fn connection_without_db_string(&self) -> String{
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError>{
    //Get a default, or a reader, config
    let mut settings = config::Config::default();

    //read and parse the configuration file "config"
    settings.merge(config::File::with_name("config"));

    //convert settings in our setting
    settings.try_into()

}