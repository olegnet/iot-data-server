use actix_web::{get, HttpResponse, post, Responder, web};
use actix_web::http::StatusCode;
use deadpool_postgres::{Pool};

use crate::postgres::get_latest_temperature;
use crate::postgres::insert_temperature;

pub mod config;
pub mod postgres;

#[get("/")]
pub async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[get("/sensor/{sensor_id}")]
pub async fn sensor_get(sensor_id: web::Path<i32>, db_pool: web::Data<Pool>) -> impl Responder {
    let sensor_id = sensor_id.into_inner();
    let result = get_latest_temperature(&db_pool, sensor_id)
        .await;
    match result {
        Ok(sensor) => format!("sensor_id={} temperature={} time={:?}\n", sensor_id, sensor.temperature, sensor.time)
            .with_status(StatusCode::OK),
        Err(e) => format!("{}\n", e)
            .with_status(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

#[post("/sensor/{sensor_id}/{temperature}")]
pub async fn sensor_post(path: web::Path<(i32, f32)>, db_pool: web::Data<Pool>) -> impl Responder {
    let (sensor_id, temperature) = path.into_inner();
    let result = insert_temperature(&db_pool, sensor_id, temperature)
        .await;
    match result {
        Ok(v) => format!("result={}\n", v)
            .with_status(StatusCode::OK),
        Err(e) => format!("{}\n", e)
            .with_status(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
