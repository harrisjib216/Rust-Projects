use crate::models::Status;
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::Pool;

pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "ok".to_string(),
    })
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {
    let client = db_pool.get().await.expect("Error connecting to db.");
}
