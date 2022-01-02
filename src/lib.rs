use actix_web::dev::Server;
use actix_web::{post, web, App, HttpResponse, HttpServer};
use std::net::TcpListener;
use serde::{Serialize, Deserialize};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Serialize, Deserialize)]
pub struct Board {
    pub cells: Vec::<bool>
}

#[post("/reset")]
async fn reset() -> HttpResponse {
    let board = Board { cells: vec![true, false, false] };
    HttpResponse::Ok().json(board)
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .service(reset)
        })
        .listen(listener)?
        .run();
    Ok(server)
}
