use serde::{Serialize,Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)]
pub struct Status {
    pub status:String
}

<<<<<<< HEAD
#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="person")]
pub struct Persona{
    ci:String,
    name: String,
    genero: String,
    estado_civil: String,
    fecha_nacimiento: String,
    telefono:String,
    direccion:String,
    email:String,
    validado:bool
=======
#[derive(Serialize,Deserialize, PostgresMapper)]
#[pg_mapper(table="person")]
pub struct Persona{
    ci:String,
    nombre: String,
    genero: String,
    estado_civil: String,
    fecha_nac: String,
    telefono:String,
    direccion:String,
    email:String,
    validado:String
>>>>>>> Desarrollo
}