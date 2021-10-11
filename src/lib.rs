use actix_web::{get, HttpResponse, post, Responder, Result, web};
use deadpool_postgres::Pool;

use postgres::get_latest_temperature;

pub mod config;
pub mod postgres;

#[get("/")]
pub async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[get("/sensor/{sensor_id}")]
pub async fn sensor_get(sensor_id: web::Path<i32>, db_pool: web::Data<Pool>) -> Result<String> {
    let sensor_id = sensor_id.into_inner();
    let sensor = get_latest_temperature(&db_pool, sensor_id).await.unwrap();
    Ok(format!("sensor_id={} temperature={} time={:?}\n", sensor_id, sensor.temperature, sensor.time))
}

#[post("/sensor/{sensor_id}/{sensor_key}")]
pub async fn sensor_post(sensor_id: web::Path<i32>, sensor_key: web::Path<i32>) -> Result<String> {
    Ok(format!("sensor_id={} sensor_key={}\n", sensor_id, sensor_key))
}
