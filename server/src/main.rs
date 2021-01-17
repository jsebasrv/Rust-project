mod models;
mod config;
mod handlers;
mod db;

use std::fs::File;          //im using this to work with files from filesystem
use std::path::Path;        //this is for representing path of the files
use std::io::Read;
use std::error::Error;      //
use regex::Regex;
use actix_web::{HttpServer, App, web};
use std::io;
use crate::handlers::*;
use dotenv::dotenv;
use tokio_postgres::NoTls;

extern crate regex;

struct Persona{
    ci:String,
    name: String,
    genero: String,
    estado_civil: String,
    fecha_nacimiento: String,
    telefono:String,
    direccion:String,
    email:String,
    validado:bool
}

impl Persona{
    fn validar_cedula(&self) -> bool{
        
        let cedula_regex = Regex::new(r"^(0[1-9]|1[0-9]|2[0-4]|30|50|80|99)[0-9]{8}").unwrap();
        let mut res = cedula_regex.is_match(&self.ci);
        if res == true{
            let id:i64 = self.ci.parse().unwrap();
            if id == 9999999999{ //consumidor final
                return res;
            }
            //taken each digit
            let first_n_ci = &self.ci[0..1];
            let second_n_ci = &self.ci[1..2];
            let third_n_ci = &self.ci[2..3];
            let forth_n_ci = &self.ci[3..4];
            let fifth_n_ci = &self.ci[4..5];
            let sixth_n_ci = &self.ci[5..6];
            let seventh_n_ci = &self.ci[6..7];
            let eigth_n_ci = &self.ci[7..8];
            let nine_n_ci = &self.ci[8..9];
            let last_digit = &self.ci[9..10];
            //parsing strings
            let mut temp1: i64= first_n_ci.parse().unwrap();
            let temp2: i64= second_n_ci.parse().unwrap();
            let mut temp3: i64= third_n_ci.parse().unwrap();
            let temp4: i64= forth_n_ci.parse().unwrap();
            let mut temp5: i64= fifth_n_ci.parse().unwrap();
            let temp6: i64= sixth_n_ci.parse().unwrap();
            let mut temp7: i64= seventh_n_ci.parse().unwrap();
            let temp8: i64= eigth_n_ci.parse().unwrap();
            let mut temp9: i64= nine_n_ci.parse().unwrap();
            let temp10: i64= last_digit.parse().unwrap();

            if temp3 > 6{
                res = false;
                return res;
            }
            //print!("{},{},{},{},{},{},{},{},{},{}\n",temp1,temp2,temp3,temp4,temp5,temp6,temp7,temp8,temp9,temp10);
            //calculo posiciones pares
            if (temp1 * 2) > 9{

                temp1 = temp1*2 - 9;
            }else{
                temp1 =temp1*2;
            };
            if (temp3 * 2) > 9{

                temp3 = temp3*2 - 9;
            }else {
                temp3 = temp3*2;
            };
            if (temp5 * 2) > 9{

                temp5 = temp5*2 -9;
            } else {
                temp5 = temp5*2;
            };

            if (temp7 * 2) > 9{
                temp7 = temp7*2 - 9;
            } else {
                temp7 =temp7*2;
            };

            if (temp9 * 2) > 9{

                temp9 = temp9*2 -9;
            } else {
                temp9 = temp9*2;
            };

            let impares = temp1+temp3+temp5+temp7+temp9;
            //println!("soy impares {}", impares);
            let pares = temp2+temp4+temp6+temp8;
            //println!("soy pares {}", pares);
            let suma_total = pares + impares;
            //println!("{}",suma_total);
            let modulo = suma_total % 10;
            let next_decena = (suma_total - modulo) + 10;
            //println!("{}",next_decena);
            let mut valid_digit = next_decena-suma_total;
            //println!("{}",valid_digit);

            if valid_digit == 10{
                valid_digit = 0;
            }
            
            if valid_digit == temp10{
                return true;
            }
            res =false;
        }
        return res;
    }

    fn validar_nombre(&self) -> bool {
        //Commun Person
        let nombre_regex = Regex::new(r"^[A-ZÀ-ÿ]+((?: )[A-ZÀ-ÿ]+)+").unwrap();
        let res = nombre_regex.is_match(&self.name);
        //println!("El nombre es: {}, por ende es {}", &self.name, res);
        return res;
        //Aqui puedo hacer para la Empresa
    }

    fn validar_genero(&self)-> bool {
        let genero_regex = Regex::new(r"^F|M").unwrap();
        let res = genero_regex.is_match(&self.genero);
        //println!("el genero es: {}", res);
        return res;
    }

    fn validar_estado_civil(&self)->bool{
        let estado_civil_regex = Regex::new(r"^(SOLTERO|CASADO|DIVORCIADO|VIUDO|EN UNION DE HECHO)").unwrap();
        let res = estado_civil_regex.is_match(&self.estado_civil);
        return res;
    }

    fn validar_fecha_nac(&self) -> bool{
        let fecha_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}").unwrap();
        let res = fecha_regex.is_match(&self.fecha_nacimiento);
        return res
    }

    fn validar_tel(&self) -> bool{
        

        if self.telefono.len()== 9 || self.telefono.len()== 12 {

            let telefono_regex = Regex::new(r"^((?:593)*0[2-7][0-9]{7})").unwrap();
            let res = telefono_regex.is_match(&self.telefono);
            return res
        }else if self.telefono.len()== 10 {
            //celular
            let celular_regex = Regex::new(r"^09[0-9]{8}").unwrap();
            let res = celular_regex.is_match(&self.telefono);
            return res
        }else {
            return false;
        }
    }

    fn validar_direccion(&self) -> bool{
        let address_regex = Regex::new(r"^[a-z0-9A-Z]+(?: *[-a-z0-9A-Z]+)+").unwrap();
        let res = address_regex.is_match(&self.direccion);
        return res
    }

    fn validar_email(&self) -> bool{
        let email_regex = Regex::new(r"^([a-z0-9_\-+]([a-z0-9_\-+.]*[a-z0-9_\-+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
        let res = email_regex.is_match(&self.email);
        return res
    }
}

#[actix_rt::main]
async fn main() -> io::Result<()>{
    
    let data_path = Path::new("data/Personas.csv");
    let mut file = match File::open(data_path){
        Err(e) => panic!("No pude abrir {} :{}", data_path.display(),e.description()),
        Ok(file) => file
    };

    
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Can't read");
    
    let mut vector_data = Vec::new();

    match file.read_to_end(&mut vector_data){
        Err(e) => panic!("No pude leer {} :{}", data_path.display(),e.description()),
        Ok(bytes) => println!("Lectura exitosa, fueron {} bytes del archivo {}", bytes, data_path.display()),
    };

    /*
    for n in 1..vector_data.len(){
        println!("{}",vector_data[n]);
    }*/


    let lines: Vec<&str> = contents.lines().collect();
    for i in 0..lines.len(){
        println!("{}",lines[i]);
    }

    let v: Vec<&str> = contents.split(';').collect();
    /*
    for i in 0..lines.len(){
        for j in 0..lines.len(){
            v[j] = lines[j].split(';').collect();
        }
    }

    
    for i in 1..v.len(){
        println!("{}",v[i]);
    }*/
    
    let ci:String = v[0].to_string();
    let name: String = v[1].to_string();
    let genero: String = v[2].to_string();
    let estado_civil: String =v[3].to_string();
    let fecha_nacimiento: String = v[4].to_string();
    let telefono:String =v[5].to_string();
    let direccion:String = v[6].to_string();
    let email:String= v[7].to_string();
    let validado:bool = false;

    let per = Persona {
        ci: ci,
        name:name,
        genero:genero,
        estado_civil:estado_civil,
        fecha_nacimiento:fecha_nacimiento,
        telefono: telefono,
        direccion: direccion,
        email:email,
        validado: validado,
    };


    /*println!("{},{},{},{},{},{},{},{},{}", per.ci, per.name, per.genero, per.estado_civil, per.fecha_nacimiento,
    per.telefono,per.direccion,per.email,per.validado);*/

    let mut observacion = "".to_string();

    if per.validar_cedula() & per.validar_nombre() & per.validar_genero() & per.validar_estado_civil() & per.validar_fecha_nac() & per.validar_tel() & per.validar_direccion() & per.validar_email(){
        println!("Es valido");
        observacion +="Es valido\n";
        per.validado == true;   // cambiando es estado de validacion
    }else {
        if per.validar_cedula() == false{
            observacion+="No Cumple los requisitos de la cédula\n";
        }else if per.validar_nombre() == false {
            observacion+="No Cumple los requisitos del nombre\n";
        }else if per.validar_genero() == false{
            observacion+="No Cumple los requisitos del genero\n";
        }else if per.validar_estado_civil() == false{
            observacion+="No Cumple los requisitos del Estado Civil\n";
        }
        else if per.validar_fecha_nac() == false{
            observacion+="No Cumple los requisitos de la fecha de nacimiento\n";
        }
        else if per.validar_tel() == false {
            observacion+="No Cumple los requisitos del telefono\n";
        }
        else if per.validar_direccion() == false{
            observacion+="No Cumple los requisitos de la direccion\n";
        }
        else if per.validar_email() == false{
            observacion+="No Cumple los requisitos del email\n";
        }
        println!("{}",observacion);
    }

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