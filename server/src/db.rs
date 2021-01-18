use crate::models::{Persona};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;


pub async fn get_personas(client: &Client)-> Result<Vec<Persona>,io::Error>{

    let statement = client.prepare("select * from person").await.unwrap();

    let personas = client.query(&statement, &[])
        .await
        .expect("Error getting person list")
        .iter()
        .map(|row| Persona::from_row_ref(row).unwrap())
        .collect::<Vec<Persona>>();

    Ok(personas)
}

//debo crear una funcion que haga la insercion