pub mod config;

use actix_web::{get, post, web, HttpResponse, Responder, Result};

#[get("/")]
pub async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[get("/sensor/{sensor_id}")]
pub async fn sensor_get(web::Path(sensor_id): web::Path<u32>) -> Result<String> {
    println!("GET /sensor/{}", sensor_id);
    Ok(format!("sensor_id={}", sensor_id))
}

#[post("/sensor/{sensor_id}/{sensor_key}")]
pub async fn sensor_post(web::Path((sensor_id, sensor_key)): web::Path<(u32, u32)>) -> Result<String> {
    println!("POST /sensor/{}/{}", sensor_id, sensor_key);
    Ok(format!("sensor_id={} sensor_key={}", sensor_id, sensor_key))
}