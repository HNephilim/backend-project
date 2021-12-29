
use backend_project::{startup, configuration};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = configuration::get_configuration().expect("Failed to read configuration");
    let conn_pool = sqlx::PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to PostgresDB");

    let address = format!("127.0.0.1:{}", configuration.app_port);
    let listener = std::net::TcpListener::bind(address)?;
    println!("Port = {}", &listener.local_addr().unwrap().port());
    startup::run_server(listener, conn_pool)?.await
}
