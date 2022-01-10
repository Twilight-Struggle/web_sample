pub mod telemetry;

use actix_web::dev::Server;
use actix_web::{post, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::net::TcpListener;
mod core;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

struct GameManeger {
    games: Mutex<HashMap<Uuid, core::Board>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MakeResult {
    pub id: Uuid,
    pub board: core::Board,
    pub res: String,
}

#[allow(clippy::async_yields_async)]
#[tracing::instrument(
    skip(data),
    fields(
        request_id = %Uuid::new_v4(),
    )
)]
#[post("/make")]
async fn make(data: web::Data<GameManeger>) -> HttpResponse {
    let mut games = data.games.lock().unwrap();
    let new_id = Uuid::new_v4();
    let board = core::Board::new();
    games.insert(new_id, board.clone());
    HttpResponse::Ok().json(MakeResult {
        id: new_id,
        board,
        res: "made".to_string(),
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub id: Uuid,
    pub from: usize,
    pub to: usize,
}

#[allow(clippy::async_yields_async)]
#[tracing::instrument(
    skip(data),
    fields(
        request_id = %Uuid::new_v4(),
    )
)]
#[post("/reset")]
async fn reset(info: web::Json<Info>, data: web::Data<GameManeger>) -> HttpResponse {
    let mut games = data.games.lock().unwrap();
    match games.get(&info.id) {
        Some(board) => {
            let board = board.reset();
            games.insert(info.id, board.clone());
            HttpResponse::Ok().json(MakeResult {
                id: info.id,
                board,
                res: "reseted".to_string(),
            })
        }
        None => HttpResponse::BadRequest().finish(),
    }
}

#[allow(clippy::async_yields_async)]
#[tracing::instrument(
    skip(data),
    fields(
        request_id = %Uuid::new_v4(),
    )
)]
#[post("/mov")]
async fn mov(info: web::Json<Info>, data: web::Data<GameManeger>) -> HttpResponse {
    let mut games = data.games.lock().unwrap();
    match games.get(&info.id) {
        Some(board) => match board.idou(info.from, info.to) {
            Some(board_next) => {
                games.insert(info.id, board_next.clone());
                let goaled = if board_next.goaled() {
                    "Goal!"
                } else {
                    "OK move"
                };
                HttpResponse::Ok().json(MakeResult {
                    id: info.id,
                    board: board_next,
                    res: goaled.to_string(),
                })
            }
            None => {
                let goaled = if board.goaled() { "Goal!" } else { "NG move" };
                HttpResponse::Ok().json(MakeResult {
                    id: info.id,
                    board: board.clone(),
                    res: goaled.to_string(),
                })
            }
        },
        None => HttpResponse::BadRequest().finish(),
    }
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let gamemaneger = web::Data::new(GameManeger {
        games: Mutex::new(HashMap::new()),
    });
    let server = HttpServer::new(move || {
        App::new()
            .app_data(gamemaneger.clone())
            .route("/health_check", web::get().to(health_check))
            .service(make)
            .service(reset)
            .service(mov)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
