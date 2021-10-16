use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;

use iot_data_server::{root, sensor_get, sensor_post};
use iot_data_server::config::{Config, DEFAULT_CONFIG_NAME};
use iot_data_server::postgres::Postgres;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let config = Config::read_from(DEFAULT_CONFIG_NAME.to_string())
        .unwrap();

    let bind_config = config.bind
        .expect("No 'bind' record in config file");

    let postgres_config = config.postgres
        .expect("No 'postgres' record in config file");

    let pool = Postgres::new_pool(postgres_config)
        .unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(root)
            .service(sensor_get)
            .service(sensor_post)
            .wrap(Logger::default())
    });

    let server = bind_config.iter()
        .fold(Ok(server), |server, bind_item| {
            server?.bind(bind_item)
        });

    server?.run()
        .await
}
