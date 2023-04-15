# ExportPicturesRUST
Utilitário para exportar as imagens gravadas na tabela do Banco de Dados

## Steps to run the project
- set DATABASE_URL in the .env file
- start postgres daemon/service   or  use DOCKER with the official POSTGRES image
- cargo build
- cargon run

## Build & Usage
- the rust compiler needs to connect to database while expanding SQLx macros, to achieve this the .env file must be
set before building the project
- the postgres daemon/service must be running to compile the project
