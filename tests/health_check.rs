use reqwest;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
	// Arrange
	let port = spawn_app();
	let client = reqwest::Client::new();

	// Act
	let response = client
		.get(format!("http://127.0.0.1:{}/health_check", port))
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
	let port = spawn_app();
	let client = reqwest::Client::new();

	// Act
	let response = client
		.get(format!("http://127.0.0.1:{}/", port))
		.send()
		.await
		.expect("Failed to execute request.");

	// Response
	assert!(response.status().is_success());
	assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> u16 {
	let address = "127.0.0.1:0";

	let listener = TcpListener::bind(address).expect("Failed to bind random port");
	let port = listener.local_addr().unwrap().port();

	let server = zero2prod::run(listener).expect("Failed to bind address");
	let _ = tokio::spawn(server);

	port
}
