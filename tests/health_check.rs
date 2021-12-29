use sqlx::Connection;

#[tokio::test]
async fn health_check_works() {
    //Arrange
    let addrs = spawn_app();

    let client = reqwest::Client::new();

    //Act
    let response = client
        .get(&format!("{}/health_check", &addrs))
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
    let addrs = spawn_app();
    let config = backend_project::configuration::get_configuration().expect("Failed to read config file");
    let mut connection = sqlx::PgConnection::connect(&config.database.connection_string()).await.expect("Failed to connect to Postgres");
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    //Act
    let response = client
        .post(&format!("{}/subscription", &addrs))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    //Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email,name FROM subscription")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_return_400_when_data_is_missing() {
    //Arrange
    let addrs = spawn_app();
    let client = reqwest::Client::new();
    let test_case = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_case {
        //Act
        let response = client
            .post(&format!("{}/subscription", &addrs))
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

fn spawn_app() -> String {
    let listener =
        std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");

    //Get the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();

    let server = backend_project::startup::run_server(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
