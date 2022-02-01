use std::net::TcpListener;
use web_sample::configuration::get_configuration;
use web_sample::run;
use web_sample::telemetry::init_subscriber;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_subscriber(std::io::stdout);
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("{}:{}", configuration.host, configuration.port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
