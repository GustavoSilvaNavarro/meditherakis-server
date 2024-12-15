mod routes;
use actix_web::{middleware::Logger, web::scope, App, HttpServer};
mod config;
use log::info;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = config::read_config();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i %r %s %b %T"))
            .service(
                scope("/meditherakis/api")
                    .configure(routes::monitoring_routes)
                    .configure(routes::quote_routes),
            )
    });

    info!(
        "🚀 Starting server at http://{}:{}",
        config.host, config.port
    );

    server
        .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await
}
