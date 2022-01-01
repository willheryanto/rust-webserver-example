use netflux_service::settings::get_settings;
use netflux_service::database::init_pool;
use netflux_service::server::run;
use netflux_service::telemetry::{get_subscriber, init_subscriber};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("netflux-service".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let settings = get_settings().expect("failed to get settings");

    let connection_pool = init_pool(settings.database.with_db());
    let listener = TcpListener::bind(settings.application.address_without_protocol())?;

    run(listener, connection_pool)?.await?;
    Ok(())
}
