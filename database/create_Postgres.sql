
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


-- -----------------------------------------------------
-- Data for table `commercedb`.`fornecedor`
-- -----------------------------------------------------
INSERT INTO fornecedor(id, cnpj, nome, email) VALUES (1, '06.347.409/0069-53', 'Lojas Centauro', 'sac@centauro.com.br');
INSERT INTO fornecedor(id, cnpj, nome, email) VALUES (2, '05.570.714/0001-59', 'Kabum Comércio Eletrônico', 'faleconosco@kabum.com.br');
INSERT INTO fornecedor(id, cnpj, nome, email) VALUES (3, '09.339.936/0001-16', 'Netshoes Comércio Eletrônico', 'sac@netshoes.com.br');

-- -----------------------------------------------------
-- Data for table `commercedb`.`categoria`
-- -----------------------------------------------------
INSERT INTO categoria(id, nome) VALUES (1, 'Móveis');
INSERT INTO categoria(id, nome) VALUES (2, 'Eletrônicos');
INSERT INTO categoria(id, nome) VALUES (3, 'Relógios');
INSERT INTO categoria(id, nome) VALUES (4, 'Celulares');
INSERT INTO categoria(id, nome) VALUES (5, 'Artigos Esportivos');
