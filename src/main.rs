use backend_project::{startup, configuration};
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //Setup logger
    LogTracer::init().expect("Failed to set logger");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formating_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formating_layer);
    set_global_default(subscriber).expect("Faield to set a subscriber");

    let configuration = configuration::get_configuration().expect("Failed to read configuration");
    let conn_pool = sqlx::PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to PostgresDB");

    let address = format!("127.0.0.1:{}", configuration.app_port);
    let listener = std::net::TcpListener::bind(address)?;
    println!("Port = {}", &listener.local_addr().unwrap().port());
    startup::run_server(listener, conn_pool)?.await
}
