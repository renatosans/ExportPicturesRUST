
create database commercedb;
use commercedb;

-- public.categoria definition

-- Drop table

-- DROP TABLE public.categoria;

CREATE TABLE public.categoria (
	id serial4 NOT NULL,
	nome varchar(120) NOT NULL,
	CONSTRAINT categoria_pkey PRIMARY KEY (id)
);


-- public.fornecedor definition

-- Drop table

-- DROP TABLE public.fornecedor;

CREATE TABLE public.fornecedor (
	id serial4 NOT NULL,
	cnpj varchar(45) NOT NULL,
	nome varchar(250) NOT NULL,
	email varchar(120) NULL,
	CONSTRAINT fornecedor_pkey PRIMARY KEY (id)
);


-- public.produto definition

-- Drop table

-- DROP TABLE public.produto;

CREATE TABLE public.produto (
	id serial4 NOT NULL,
	nome varchar(255) NOT NULL,
	preco DOUBLE PRECISION NOT NULL,
	categoria int4 NULL,
	fornecedor int4 NULL,
	descricao varchar(255) NULL,
	foto text NULL,
	"formatoImagem" varchar(45) NULL,
	"dataCriacao" timestamp(0) NULL DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT produto_pkey PRIMARY KEY (id)
);


-- public."unidadeMedida" definition

-- Drop table

-- DROP TABLE public."unidadeMedida";

CREATE TABLE public."unidadeMedida" (
	id serial4 NOT NULL,
	descricao varchar(65) NOT NULL,
	sigla varchar(10) NULL,
	CONSTRAINT "unidadeMedida_pkey" PRIMARY KEY (id)
);
