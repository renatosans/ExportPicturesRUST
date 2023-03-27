// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]


use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::produto;
use serde::{Serialize, Deserialize};


#[derive(Queryable, Debug)]
pub struct Categoria {
    pub id: i32,
    pub nome: String,
}

#[derive(Queryable, Debug)]
pub struct Fornecedor {
    pub id: i32,
    pub cnpj: String,
    pub nome: String,
    pub email: Option<String>,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = produto)]
pub struct Produto {
    pub id: i32,
    pub nome: String,
    pub preco: f64,
    pub categoria: Option<i32>,
    pub fornecedor: Option<i32>,
    pub descricao: Option<String>,
    pub foto: Option<String>,
    #[diesel(column_name = formatoImagem)]
    pub formato_imagem: Option<String>,
    #[diesel(column_name = dataCriacao)]
    pub data_criacao: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug)]
pub struct Unidademedida {
    pub id: i32,
    pub descricao: String,
    pub sigla: Option<String>,
}
