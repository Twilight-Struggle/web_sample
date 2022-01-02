use actix_web::dev::Server;
use actix_web::{post, web, App, HttpResponse, HttpServer};
use std::net::TcpListener;
use serde::{Serialize, Deserialize};
mod core;
use std::sync::Mutex;
use uuid::Uuid;
use std::collections::HashMap;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

struct GameManeger {
    games: Mutex<HashMap<Uuid, core::Board>>
}

#[post("/make")]
async fn make(data: web::Data<GameManeger>) -> HttpResponse {
    let mut games = data.games.lock().unwrap();
    let new_id = Uuid::new_v4();
    (*games).insert(new_id, core::Board::new());
    HttpResponse::Ok().json(new_id)
}

#[post("/reset")]
async fn reset(data: web::Data<core::Board>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let gamemaneger = web::Data::new(GameManeger {
        games: Mutex::new(HashMap::new())
    });
    let server = HttpServer::new(move || {
        App::new()
            .app_data(gamemaneger.clone())
            .route("/health_check", web::get().to(health_check))
            .service(reset)
            .service(make)
        })
        .listen(listener)?
        .run();
    Ok(server)
}
