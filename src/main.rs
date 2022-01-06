use secrecy::ExposeSecret;
use backend_project::{startup, configuration, telemetry};


#[tokio::main]
async fn main() -> std::io::Result<()> {

    let subscriber = telemetry::get_subscriber("Zero2Prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);



    let configuration = configuration::get_configuration().expect("Failed to read configuration");
    let conn_pool = sqlx::PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to PostgresDB");

    let address = format!("127.0.0.1:{}", configuration.app_port);
    let listener = std::net::TcpListener::bind(address)?;
    println!("Port = {}", &listener.local_addr().unwrap().port());
    startup::run_server(listener, conn_pool)?.await
}


