-- Database: rust

-- DROP DATABASE rust;

CREATE DATABASE rust
    WITH 
    OWNER = postgres
    ENCODING = 'UTF8'
    LC_COLLATE = 'English_United States.1252'
    LC_CTYPE = 'English_United States.1252'
    TABLESPACE = pg_default
    CONNECTION LIMIT = -1;

drop index if exists PERSON.PERSON_PK;

drop table if exists PERSON;

create table PERSON 
(
   CI                   varchar(20)                    not null,
   NOMBRE               varchar(30)                    not null,
   GENERO               varchar(1)                     not null,
   ESTADO_CIVIL         varchar(20)                    not null,
   FECHA_NAC            varchar(20)                    not null,
   TELEFONO             varchar(12)                    not null,
   DIRECCION            varchar(50)                    not null,
   EMAIL                varchar(50)                    not null,
   VALIDADO             varchar(1)                     not null,
   OBSERVACION          varchar(100)                   not null,
   constraint PK_PERSON primary key (CI)
);

create unique index PERSON_PK on PERSON (
CI ASC
);

