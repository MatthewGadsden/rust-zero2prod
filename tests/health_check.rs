use reqwest;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
	// Arrange
	let app_address = spawn_app();
	let client = reqwest::Client::new();

	// Act
	let response = client
		.get(format!("{}/health_check", app_address))
		.send()
		.await
		.expect("Failed to execute request.");

	// Response
	assert!(response.status().is_success());
	assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn api_base_works() {
	// Arrange
	let app_address = spawn_app();
	let client = reqwest::Client::new();

	// Act
	let response = client
		.get(format!("{}/", app_address))
		.send()
		.await
		.expect("Failed to execute request.");

	// Response
	assert!(response.status().is_success());
	assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
	// Arrange
	let app_address = spawn_app();
	let client = reqwest::Client::new();

	// Act
	let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
	let response = client
		.post(&format!("{}/subscriptions", &app_address))
		.header("Content-Type", "application/x-www-form-urlencoded")
		.body(body)
		.send()
		.await.expect("Failed to execute request.");
	
	// Assert
	assert_eq!(200, response.status().as_u16());
}

fn spawn_app() -> String {
	let app_address = "127.0.0.1:0";

	let listener = TcpListener::bind(app_address).expect("Failed to bind random port");
	let port = listener.local_addr().unwrap().port();

	let server = zero2prod::run(listener).expect("Failed to bind address");
	let _ = tokio::spawn(server);

	format!("http://127.0.0.1:{}", port)
}
