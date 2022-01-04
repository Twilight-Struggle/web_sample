use std::net::TcpListener;
use anisoc::run;
use tracing_subscriber::EnvFilter;
use tracing_subscriber;
use tracing_subscriber::fmt::format::FmtSpan;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("Debug"));
    let format = tracing_subscriber::fmt::format().pretty();
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .event_format(format)
        .init();
    let address = TcpListener::bind("127.0.0.1:8000")?;
    run(address)?.await
}
