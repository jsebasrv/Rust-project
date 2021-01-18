mod models;
mod config;
mod handlers;
mod db;

use crate::models::Persona;
use actix_web::{HttpServer, App, web};
use std::io;
use crate::handlers::*;
use dotenv::dotenv;
use tokio_postgres::NoTls;

#[actix_rt::main]
async fn main() -> io::Result<()>{ //async y esto va despues del main 

    models::carga_datos();
    
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();
    

    //creacion del server
    HttpServer::new(move || {

        App::new()
            .data(pool.clone())
            .route("/",web::get().to(status))
            .route("/personas{_:/?}", web::get().to(get_personas))

    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await

}