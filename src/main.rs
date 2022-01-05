use std::net::TcpListener;
use anisoc::run;
use anisoc::telemetry::init_subscriber;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_subscriber(std::io::stdout);
    let address = TcpListener::bind("127.0.0.1:8000")?;
    run(address)?.await
}
