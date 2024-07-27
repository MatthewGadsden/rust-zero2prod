use std::net::TcpListener;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
	const ADDRESS: &str = "127.0.0.1:5000";
	let listener = TcpListener::bind(ADDRESS).expect("Failed to bind random port");

	run(listener)?.await
}
