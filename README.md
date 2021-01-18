# Rust-project

El proyecto es desarrollado con Rust, y usamos el framework Actrix y la base de datos Postgres.

En el repositorio se encuentran todos los archivos necesarios para ejecutar el proyecto.

Requisitos:

Necesitas tener instalado Postgres, Docker, Rust y dentro de este installar Cargo.
El archivo .env debe ser configurado segun el nombre del usuario de postgres y la contraseña


Pasos:
Ingresar a la carpeta data y tomar el archivo persona.sql y crear al base de datos en postgres

Una vez creada la base de datos tomar el archivo "inserciones manuales.txt" y correr el script dentro de la base de datos creada

Luego por medio de la línea de comando ir a la ubicación del proyecto, a la carpera "server" y ejecutar el comando cargo run.

crear el encapsulamiento de todo se debe correr el siguiente comando:
Luego ejecutar el comando docker-compose up -d
