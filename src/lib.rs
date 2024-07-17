use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
	HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
	_email: String,
	_name: String,
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
	HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
	let server = HttpServer::new(|| {
		App::new()
			.route("/health_check", web::get().to(health_check))
			.route("/subscriptions", web::post().to(subscribe))
			.route("/", web::get().to(health_check))
	})
	.listen(listener)?
	.run();

	Ok(server)
}
