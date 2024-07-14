use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
	HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
	HttpServer::new(|| {
		App::new()
			.route("/healthCheck", web::get().to(health_check))
	})
	.bind("127.0.0.1:8000")?
	.run()
	.await
}
