use crate::models::Status;
use crate::db;
<<<<<<< HEAD
use actix_web::{web,Responder, HttpResponse};
use deadpool_postgres::{Pool,Client};

pub async fn status() -> impl Responder{
    web::HttpResponse::Ok()
        .json(Status {status: "UP".to_string()})
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
=======
use actix_web::{web, Responder, HttpResponse};
use deadpool_postgres::{Pool,Client};


pub async fn status() -> impl Responder{
     web::HttpResponse::Ok()
        .json(Status {status: "Ok".to_string()})
}

pub async fn get_personas(db_pool: web::Data<Pool>) -> impl Responder{
    let client:Client = 
        db_pool.get().await.expect("Error connecting to database");
    
    let result = db::get_personas(&client).await;

    match result{
        Ok(personas) => HttpResponse::Ok().json(personas),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}
>>>>>>> Desarrollo
