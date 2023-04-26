mod libs;

use dotenv::dotenv;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();

	let port = std::env::var("PORT")
		.unwrap_or("3000".to_string())
		.parse::<u16>().unwrap();

	HttpServer::new(|| {
		App::new().service(hello)
	})
	.bind(("127.0.0.1", port))?
	.run()
	.await
}

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body(libs::metrics::get_metrics())
}

