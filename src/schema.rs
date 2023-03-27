// @generated automatically by Diesel CLI.

diesel::table! {
    categoria (id) {
        id -> Integer,
        nome -> Varchar,
    }
}

diesel::table! {
    fornecedor (id) {
        id -> Integer,
        cnpj -> Varchar,
        nome -> Varchar,
        email -> Nullable<Varchar>,
    }
}

diesel::table! {
    produto (id) {
        id -> Integer,
        nome -> Varchar,
        preco -> Float8,
        categoria -> Nullable<Integer>,
        fornecedor -> Nullable<Integer>,
        descricao -> Nullable<Varchar>,
        foto -> Nullable<Longtext>,
        formatoImagem -> Nullable<Varchar>,
        dataCriacao -> Nullable<Timestamp>,
    }
}

diesel::table! {
    unidademedida (id) {
        id -> Integer,
        descricao -> Varchar,
        sigla -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    categoria,
    fornecedor,
    produto,
    unidademedida,
);
