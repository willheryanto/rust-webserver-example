//use sqlx::{Executor, PgPool};
//use reqwest::Client;
//use uuid::Uuid;
//use netflux_service::settings::{get_settings, DatabaseSettings};
//use netflux_service::database::init_pool;
//use netflux_service::server::run;
//use netflux_service::telemetry::{get_subscriber, init_subscriber};

//use once_cell::sync::Lazy;

//static TRACING: Lazy<()> = Lazy::new(|| {
    //let default_filter_level = "info".to_string();
    //let subscriber_name = "test".to_string();

    //if std::env::var("TEST_LOG").is_ok() {
        //let subscriber = get_subscriber(subscriber_name, default_filter_level.into(), std::io::stdout);
        //init_subscriber(subscriber);
    //} else {
        //let subscriber = get_subscriber(subscriber_name, default_filter_level.into(), std::io::sink);
        //init_subscriber(subscriber);
    //}
//});

//pub struct TestService {
    //pub address: String,
    //pub db_pool: PgPool,
//}

//pub struct TestApp {
    //pub app: TestService,
    //pub client: Client,
//}

//pub async fn configure_database(settings: &DatabaseSettings) -> PgPool {
    //// Initiate database if not exist
    //init_pool(settings.without_db())
        //.execute(&*format!(r#"CREATE DATABASE "{}";"#, settings.database_name))
        //.await
        //.expect("Failed to create database.");

    //let connection_pool = init_pool(settings.with_db());
    //sqlx::migrate!("./migrations").run(&connection_pool).await.expect("Failed to migrate database");

    //connection_pool
//}

//async fn spawn_service() -> TestService {
    //Lazy::force(&TRACING);

    //let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    //let port = listener.local_addr().unwrap().port();
    //let address = format!("http://127.0.0.1:{}", port);

    //let mut settings = get_settings().expect("Failed to get settings");
    //// Inject database name to always use fresh database for tests
    //settings.database.database_name = Uuid::new_v4().to_string();

    //let connection_pool = configure_database(&settings.database).await;

    //let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    //tokio::spawn(server);

    //TestService {
        //address,
        //db_pool: connection_pool
    //}
//}

//async fn init_test_app() -> TestApp {
    //let app = spawn_service().await;
    //let client = Client::new();

    //TestApp {
        //app,
        //client,
    //}
//}

//#[tokio::test]
//async fn liveness_test() {
    //let app = spawn_service().await;

    //let client = reqwest::Client::new();
    //let response = client
        //.get(format!("{}/healthcheck/liveness", &app.address))
        //.send()
        //.await
        //.expect("Failed to execute request");

    //assert!(response.status().is_success())
//}

//#[tokio::test]
//async fn movies_returns_a_200_for_valid_form_data() {
    //let TestApp {app, client} = init_test_app().await;
    //let body = "title=Venom:%20Let%20There%20Be%20Carnage&release_date=2021-09-30&poster_path=/rjkmN1dniUHVYAtwuV3Tji7FsDO.jpg&vote_average=7.2&vote_count=4337";

    //let res = client
        //.post(format!("{}/movies", &app.address))
        //.header("Content-Type", "application/x-www-form-urlencoded")
        //.body(body)
        //.send()
        //.await
        //.expect("failed to execute request");


    //assert_eq!(res.status().as_u16(), 200);

    //let saved = sqlx::query!(r"SELECT title, poster_path FROM movies")
        //.fetch_one(&app.db_pool)
        //.await
        //.expect("Failed to fetch movie");

    //assert_eq!(saved.title, "Venom: Let There Be Carnage");
    //assert_eq!(saved.poster_path, "/rjkmN1dniUHVYAtwuV3Tji7FsDO.jpg");
//}

//#[tokio::test]
//async fn movies_returns_a_200_for_valid_search_query() {
    //let TestApp {app, client} = init_test_app().await;

    //let res = client
        //.get(format!("{}/movies", &app.address))
        //.query(&[("movie_title", "the matrix")])
        //.send()
        //.await
        //.expect("failed to execute request");


    //assert_eq!(res.status().as_u16(), 200);
//}

//#[tokio::test]
//async fn movies_returns_a_400_when_data_is_missing() {
    //let TestApp {app, client} = init_test_app().await;
    //let test_cases = vec![
        //("&release_date=2021-09-30&poster_path=/rjkmN1dniUHVYAtwuV3Tji7FsDO.jpg&vote_average=7.2&vote_count=4337", "missing title"),
        //("title=Venom:%20Let%20There%20Be%20Carnage&poster_path=/rjkmN1dniUHVYAtwuV3Tji7FsDO.jpg&vote_count=4337", "missing release_date"),
        //("title=Venom:%20Let%20There%20Be%20Carnage&release_date=2021-09-30&vote_count=4337", "missing poster_path"),
        //("", "missing everything")
    //];

    //for (invalid_body, error_message) in test_cases {
        //let res = client
            //.post(format!("{}/movies", &app.address))
            //.header("Content-Type", "application/x-www-form-urlencoded")
            //.body(invalid_body)
            //.send()
            //.await
            //.expect(error_message);

        //assert_eq!(
            //400,
            //res.status().as_u16(),
            //"The API did not fail with 400 Bad Request when the payload was {}",
            //error_message
        //);
    //}
//}

//#[tokio::test]
//async fn movie_returns_a_400_when_fields_are_present_but_invalid() {
    //let TestApp {app, client} = init_test_app().await;
    //let test_cases = vec![
        //("title=&release_date=2021-09-30&poster_path=/rjkmN1dniUHVYAtwuV3Tji7FsDO.jpg&vote_average=7.2&vote_count=4337", "empty title"),
    //];

    //for (body, description) in test_cases {
        //let response = client
            //.post(&format!("{}/movies", &app.address))
            //.header("Content-Type", "application/x-www-form-urlencoded")
            //.body(body)
            //.send()
            //.await
            //.expect("Failed to execute request.");

        //// Assert
        //assert_eq!(
            //400,
            //response.status().as_u16(),
            //"The API did not return a 400 Bad Request when the payload was {}.",
            //description
        //);
    //}
//}

//#[tokio::test]
//async fn movies_returns_a_400_for_invalid_search_query() {
    //let TestApp {app, client} = init_test_app().await;

    //let res = client
        //.get(format!("{}/movies", &app.address))
        //.query(&[("title", "the matrix")])
        //.send()
        //.await
        //.expect("failed to execute request");


    //assert_eq!(res.status().as_u16(), 400);
//}
