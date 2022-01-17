use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::ConnectOptions;
use sqlx::postgres::{PgConnectOptions, PgSslMode};


#[derive(serde::Deserialize)]
pub struct Settings{
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings{
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings{
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database: String,
    pub require_ssl: bool,
}

impl DatabaseSettings{
    pub fn with_db(&self) -> PgConnectOptions{
        let mut option = self.without_db()
            .database(&self.database);

        option.log_statements(tracing::log::LevelFilter::Trace);

        option
    }

    pub fn without_db(&self) -> PgConnectOptions{
        let ssl_mode = if self.require_ssl{
            PgSslMode::Require
        }else{
            PgSslMode::Prefer
        };


        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)

    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError>{
    //Get a default, or a reader, config
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed tp determine the current directory");
    let configuration_directory = base_path.join("configuration");

    //Read the "default" configuration file
    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;


    //Detect the running environment
    //Defaults to Local if not specified
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    //Layer on the environment-specific values
    settings.merge(
        config::File::from(configuration_directory.join(environment.as_str())).required(true)
    )?;

    // Add in settings from environment variables (with a prefix of APP and '__' as separator)
    // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
    settings.merge(config::Environment::with_prefix("app").separator("__"))?;

    //convert settings in our setting
    settings.try_into()
}


enum Environment{
    Local,
    Production
}

impl Environment{
    pub fn as_str(&self) -> &'static str{
        match self{
            Environment::Local => {"local"}
            Environment::Production => {"production"}
        }
    }
}

impl TryFrom<String> for Environment{
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str(){
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not supported environment. Use either local or production",
                other
            ))
        }
    }
}