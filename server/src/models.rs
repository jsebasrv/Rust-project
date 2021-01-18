use std::fs::File;          //im using this to work with files from filesystem
use std::path::Path;        //this is for representing path of the files
use std::io::Read;
use std::error::Error;  
use regex::Regex;
extern crate regex;
use serde::{Serialize,Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)]
pub struct Status {
    pub mensaje:String
}

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
}

//impl Persona{
    pub fn validar_cedula(ci: String) -> bool{
        
        let cedula_regex = Regex::new(r"^(0[1-9]|1[0-9]|2[0-4]|30|50|80|99)[0-9]{8}").unwrap();
        let mut res = cedula_regex.is_match(&ci);
        if res == true{
            let id:i64 = ci.parse().unwrap();
            if id == 9999999999{ //consumidor final
                return res;
            }
            //taken each digit
            let first_n_ci = &ci[0..1];
            let second_n_ci = &ci[1..2];
            let third_n_ci = &ci[2..3];
            let forth_n_ci = &ci[3..4];
            let fifth_n_ci = &ci[4..5];
            let sixth_n_ci = &ci[5..6];
            let seventh_n_ci = &ci[6..7];
            let eigth_n_ci = &ci[7..8];
            let nine_n_ci = &ci[8..9];
            let last_digit = &ci[9..10];
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
            if (temp1 * 2) > 9{temp1 = temp1*2 - 9;}else{temp1 =temp1*2;};
            if (temp3 * 2) > 9{temp3 = temp3*2 - 9;}else {temp3 = temp3*2;};
            if (temp5 * 2) > 9{temp5 = temp5*2 -9;} else {temp5 = temp5*2;};
            if (temp7 * 2) > 9{temp7 = temp7*2 - 9;} else {temp7 =temp7*2;};
            if (temp9 * 2) > 9{ temp9 = temp9*2 -9;} else {temp9 = temp9*2;};

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

    pub fn validar_nombre(nombre: String) -> bool {
        //Commun Person
        let nombre_regex = Regex::new(r"^[A-ZÀ-ÿ]+((?: )[A-ZÀ-ÿ]+)+").unwrap();
        let res = nombre_regex.is_match(&nombre);
        //println!("El nombre es: {}, por ende es {}", &self.name, res);
        return res;
        //Aqui puedo hacer para la Empresa
    }

    pub fn validar_genero(genero: String)-> bool {
        let genero_regex = Regex::new(r"^F|M").unwrap();
        let res = genero_regex.is_match(&genero);
        //println!("el genero es: {}", res);
        return res;
    }

    pub fn validar_estado_civil(estado_civil: String)->bool{
        let estado_civil_regex = Regex::new(r"^(SOLTERO|CASADO|DIVORCIADO|VIUDO|EN UNION DE HECHO)").unwrap();
        let res = estado_civil_regex.is_match(&estado_civil);
        return res;
    }

    pub fn validar_fecha_nac(fecha_nac: String) -> bool{
        let fecha_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}").unwrap();
        let res = fecha_regex.is_match(&fecha_nac);
        return res
    }

    pub fn validar_tel(telefono: String) -> bool{
        

        if telefono.len()== 9 || telefono.len()== 12 {

            let telefono_regex = Regex::new(r"^((?:593)*0[2-7][0-9]{7})").unwrap();
            let res = telefono_regex.is_match(&telefono);
            return res
        }else if telefono.len()== 10 {
            //celular
            let celular_regex = Regex::new(r"^09[0-9]{8}").unwrap();
            let res = celular_regex.is_match(&telefono);
            return res
        }else {
            return false;
        }
    }

    pub fn validar_direccion(direccion: String) -> bool{
        let address_regex = Regex::new(r"^[a-z0-9A-Z]+(?: *[-a-z0-9A-Z]+)+").unwrap();
        let res = address_regex.is_match(&direccion);
        return res
    }

    pub fn validar_email(email: String) -> bool{
        let email_regex = Regex::new(r"^([a-z0-9_\-+]([a-z0-9_\-+.]*[a-z0-9_\-+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
        let res = email_regex.is_match(&email);
        return res
    }

//}

pub fn carga_datos(){
    let data_path = Path::new("data/Personas.csv");
    let mut file = match File::open(data_path){
        Err(e) => panic!("No pude abrir {} :{}", data_path.display(),e.description()),
        Ok(file) => file
    };

    
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Can't read");


    let lines: Vec<&str> = contents.lines().collect();
    //println!("{}",lines[0]);
    let mut vector: Vec<&str>;
    let mut v: Vec<&str>;
    for i in 0..lines.len(){
        //println!("{}",lines[i]);
        //vector.push(lines[i]);
        for j in 0..8{
            v = lines[i].split(';').collect();

            
            //vector = v[i]
            /*
            let ci:String = v[0].to_string();
            let name: String = v[1].to_string();
            let genero: String = v[2].to_string();
            let estado_civil: String =v[3].to_string();
            let fecha_nac: String = v[4].to_string();
            let telefono:String =v[5].to_string();
            let direccion:String = v[6].to_string();
            let email:String= v[7].to_string();
            let validado:String = "0".to_string();
            */
            let per = Persona{
                ci: v[0].to_string(),
                nombre:v[1].to_string(),
                genero:v[2].to_string(),
                estado_civil:v[3].to_string(),
                fecha_nac:v[4].to_string(),
                telefono: v[5].to_string(),
                direccion: v[6].to_string(),
                email:v[7].to_string(),
                validado: "0".to_string(),
            };
            


            validar_datos(per);
            /*println!("{},{},{},{},{},{},{},{},{}", per.ci, per.nombre, per.genero, per.estado_civil, per.fecha_nac,
            per.telefono, per.direccion, per.email, per.validado);*/
            println!("soy vector {}",v[j]);
        }
        
    }

}

fn validar_datos(mut per : Persona){        //, mut observacion: String
    let mut observacion :String = "No es valido".to_string();
    if validar_cedula(per.ci) & validar_nombre(per.nombre) & validar_genero(per.genero) & validar_estado_civil(per.estado_civil) & validar_fecha_nac(per.fecha_nac) & 
        validar_tel(per.telefono) & validar_direccion(per.direccion) & validar_email(per.email){
            //println!("Es valido");
        observacion = "Es valido\n".to_string();
        per.validado = "1".to_string();   // cambiando es estado de validacion
    }
    println!("{}{}",per.validado,observacion);
}

/*else if validar_cedula(per.ci) == false{
        observacion+="No Cumple los requisitos de la cédula\n";
    }else if validar_nombre(per.nombre) == false {
        observacion+="No Cumple los requisitos del nombre\n";
    }else if validar_genero(per.genero) == false{
        observacion+="No Cumple los requisitos del genero\n";
    }else if validar_estado_civil(per.estado_civil) == false{
        observacion+="No Cumple los requisitos del Estado Civil\n";
    }
    else if validar_fecha_nac(per.fecha_nac) == false{
        observacion+="No Cumple los requisitos de la fecha de nacimiento\n";
    }
    else if validar_tel(per.telefono) == false {
        observacion+="No Cumple los requisitos del telefono\n";
    }
    else if validar_direccion(per.direccion) == false{
        observacion+="No Cumple los requisitos de la direccion\n";
    }
    else if validar_email(per.email) == false{
        observacion+="No Cumple los requisitos del email\n";
    } */