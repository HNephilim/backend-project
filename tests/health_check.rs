use sqlx::{Connection, Executor, PgConnection, PgPool};

#[tokio::test]
async fn health_check_works() {
    //Arrange
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    //Act
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_return_200_for_valid_form_data() {
    //Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    //Act
    let response = client
        .post(&format!("{}/subscription", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    //Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email,name FROM subscription")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_return_400_when_data_is_missing() {
    //Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_case = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_case {
        //Act
        let response = client
            .post(&format!("{}/subscription", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        //Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}





//Ensure that 'tracing' stack is only initialised once using 'once_cell'
static TRACING: once_cell::sync::Lazy<()> = once_cell::sync::Lazy::new(||{
    use backend_project::telemetry;
    let default_name = "Test".to_string();
    let default_filter_level = "debug".to_string();

    if std::env::var("TEST_LOG").is_ok(){
        let subscriber = telemetry::get_subscriber(default_name,default_filter_level, std::io::stdout);
        telemetry::init_subscriber(subscriber);
    }else{
        let subscriber = telemetry::get_subscriber(default_name,default_filter_level, std::io::sink);
        telemetry::init_subscriber(subscriber);
    }

});

pub struct TestApp{
    pub address: String,
    pub db_pool: sqlx::PgPool,
}

async fn spawn_app() -> TestApp {
    //Runs the closure in TRACING only the first time it's invoked
    //All other invocations will skip
    once_cell::sync::Lazy::force(&TRACING);

    //Get the port assigned to us by the OS
    let listener =
        std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    //Inicialize DB Connection Pool
    let mut configurarion = backend_project::configuration::get_configuration().expect("Failed to read configurations");
    configurarion.database.db_name = uuid::Uuid::new_v4().to_string();
    let db_pool = configure_database(&configurarion.database).await;

    // Inicialize server in background worker
    let server = backend_project::startup::run_server(listener, db_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);



    TestApp{
        address,
        db_pool
    }

}

pub async fn configure_database(config: &backend_project::configuration::DatabaseSettings) -> sqlx::PgPool{
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to PostgresDB");

    //Create DB
    connection.execute(format!(r#"CREATE DATABASE "{}";"#, config.db_name).as_str())
        .await
        .expect("Failed to create database.");

    //Migrate DB
    let conn_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&conn_pool)
        .await
        .expect("failed to migrate the database");

    conn_pool

}
