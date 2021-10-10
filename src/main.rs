use std::io::{Error, ErrorKind};
use actix_web::{App, HttpServer};
use iot_data_server::{root, sensor_get, sensor_post};
use iot_data_server::config::{Config, DEFAULT_CONFIG_NAME};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::read_from(DEFAULT_CONFIG_NAME.to_string())?;

    let server = HttpServer::new(|| {
        App::new()
            .service(root)
            .service(sensor_get)
            .service(sensor_post)
    });

    let bind = config.bind
        .ok_or(Error::new(ErrorKind::Other, "No 'bind' record in config file"))?;

    let server = bind.iter()
        .fold(Ok(server), |server, bind| {
            server?.bind(bind.get())
        });

    server?.run()
        .await
}
