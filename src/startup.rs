use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes::{health_check, subscribe};

pub fn run(listener: TcpListener, connection_pool: PgPool) -> Result<Server, std::io::Error> {
	let db_pool = web::Data::new(connection_pool);

	let server = HttpServer::new(move || {
		App::new()
			.route("/health_check", web::get().to(health_check))
			.route("/subscriptions", web::post().to(subscribe))
			.route("/", web::get().to(health_check))
			.app_data(db_pool.clone())
	})
	.listen(listener)?
	.run();

	Ok(server)
}
