use crate::models::Status;
use crate::db;
use actix_web::{web,Responder, HttpResponse};
use deadpool_postgres::{Pool,Client};

pub async fn status() -> impl Responder{
    web::HttpResponse::Ok()
        .json(Status {mensaje: "para ver los datos migrados agregue en el buscador lo siguiente: /personas".to_string()})
}

pub async fn get_personas(db_pool: web::Data<Pool>) -> impl Responder{
    let client: Client =
        db_pool.get().await.expect("Error connecting to database");

    let result = db::get_personas(&client).await;

    match result {
        Ok(personas) => HttpResponse::Ok().json(personas),
        Err(_) => HttpResponse::InternalServerError().into()

    }
} 