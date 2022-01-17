use backend_project::{startup, configuration, telemetry};
use backend_project::configuration::Settings;


#[tokio::main]
async fn main() -> std::io::Result<()> {

    let subscriber = telemetry::get_subscriber("Zero2Prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read configuration");

    let conn_pool = sqlx::PgPool::connect_lazy_with(configuration.database.with_db());

    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = std::net::TcpListener::bind(address)?;
    print(&configuration);

    startup::run_server(listener, conn_pool)?.await
}

fn print(config: &Settings){
    println!("{:#?}", config);
}


