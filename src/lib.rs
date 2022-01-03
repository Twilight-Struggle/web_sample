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

#[derive(Debug, Serialize, Deserialize)]
pub struct MakeResult {
    pub id: Uuid,
    pub board: core::Board
}

#[post("/make")]
async fn make(data: web::Data<GameManeger>) -> HttpResponse {
    let mut games = data.games.lock().unwrap();
    let new_id = Uuid::new_v4();
    let board = core::Board::new();
    games.insert(new_id, board.clone());
    HttpResponse::Ok().json(MakeResult {
        id: new_id,
        board: board
    })
}

#[derive(Serialize, Deserialize)]
pub struct Info {
    pub id: Uuid,
    pub from: usize,
    pub to: usize
}

#[post("/reset")]
async fn reset(info: web::Json<Info>, data: web::Data<GameManeger>) -> HttpResponse {
    let games = data.games.lock().unwrap();
    match games.get(&info.id) {
        Some(board) => {
            let board = board.reset();
            HttpResponse::Ok().json(MakeResult {
                id: info.id,
                board: board
            })
        },
        None => HttpResponse::BadRequest().json("test")
    }
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
