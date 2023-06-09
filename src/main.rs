use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;
use std::io::{Read, Write};
use tinyfiledialogs::*;
use base64::{Engine as _, engine::general_purpose};

use dotenv::dotenv;
// use diesel::prelude::*;                       // diesel ORM
use sqlx::postgres::{PgPool, PgPoolOptions};     // sqlx

// TODO:  substituir o 'ORM Diesel' pelo 'SQLx' para remover as seguintes dependencias
//
//  By default diesel depends on the following client libraries:
//  - libpq for the PostgreSQL backend
//  - libmysqlclient for the Mysql backend
//  - libsqlite3 for the SQlite backend

use eframe::egui;
use egui::style::Margin;
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
use egui::{Color32, Direction, Frame, Pos2, RichText, Widget};

mod models;
mod schema;
use models::*;
use std::time::Duration;


/// Identifier for a custom toast kind
const MY_CUSTOM_TOAST: u32 = 0;
// pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
// pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

fn main() {
    eframe::run_native(
        "Export Pictures (Catalogo de produtos)",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(Demo::default())),
    ).unwrap();
}

struct Demo {
    duration_sec: f32,
    category: String,
    show_icon: bool,
    pool: PgPool,
    toasts: Toasts,
    toast_options: Option<ToastOptions>
}

impl Default for Demo {
    fn default() -> Self {
        dotenv().expect("Unable to load environment variables from .env file");
        let database_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

        let rt = tokio::runtime::Runtime::new().unwrap();
        let pool_options = PgPoolOptions::new()
            .max_connections(100);
        let pool = rt.block_on(pool_options.connect(&database_url))
            .expect("Unable to connect to database");

        let toasts: Toasts = Toasts::new()
        .anchor(Pos2::new(50.0, 50.0))
        .direction(Direction::TopDown)
        .align_to_end(false)
        .custom_contents(MY_CUSTOM_TOAST, my_custom_toast_contents);

        Self {
            duration_sec: 6.5,
            category: "-- Selecione --".to_string(),
            show_icon: true,
            pool: pool,
            toasts: toasts,
            toast_options: None
        }
    }
}

impl eframe::App for Demo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.options_window(ctx);

        self.toasts.show(ctx);

        ctx.request_repaint();
    }
}

impl Demo {
    fn options_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("")
            .default_pos((100.0, 100.0))
            .default_width(200.0)
            .show(ctx, |ui| {

                ui.label("Produtos: ");
                let rt = tokio::runtime::Runtime::new().unwrap();
                let categories = rt.block_on(self.retrieve_categories());

                egui::ComboBox::from_label("Categoria")
                    .selected_text(format!("{}", self.category))
                    .show_ui(ui, |ui| {
                        if categories.is_empty() { 
                            return;
                        }
                        categories.into_iter().for_each(|category: Categoria| {
                            ui.selectable_value(&mut self.category, category.nome.clone(), category.nome.clone());
                        });
                    });

                let duration = if self.duration_sec < 0.01 {
                    None
                } else {
                    Some(Duration::from_secs_f32(self.duration_sec))
                };

                let options = ToastOptions {
                    show_icon: self.show_icon,
                    ..ToastOptions::with_duration(duration)
                };

                self.toast_options = Some(options.clone());

                ui.separator();

                ui.label("Produtos(Exportar fotos): ");

                if ui.add_sized([80., 25.], egui::Button::new("Inserir")).clicked() {
                    let file_path: String;
                    let filter: Option<(&[&str], &str)> = Some((&["*.jpg", "*.gif", "*.png"], "Image Files"));
                    match open_file_dialog("Selecione a foto do produto", "*.*", filter) {
                        Some(file) => {
                            file_path = file;
                        }
                        None => return,
                    }
                    rt.block_on(self.insert_product(file_path));

                    self.toasts.add(Toast {
                        text: "Registro inserido com sucesso no banco".into(),
                        kind: ToastKind::Custom(MY_CUSTOM_TOAST),
                        options: self.toast_options.unwrap(),
                    });            
                }

                if ui.add_sized([80., 25.], egui::Button::new("Recuperar")).clicked() {
                    let output_dir = format!("{}{}", std::env::current_dir().unwrap().display(), "/img");
                    std::fs::create_dir_all(output_dir.clone()).unwrap();

                    let catalogo: Vec<Produto> = rt.block_on(self.retrieve_products());
                    if catalogo.is_empty() {
                        println!("Nenhum produto encontrado");
                        return;
                    }
                    catalogo.into_iter().for_each(|product: Produto| {
                        export_picture(product, output_dir.clone())
                    });

                    self.toasts.add(Toast {
                        text: format!("Arquivos salvos em {}", output_dir.clone()).into(),
                        kind: ToastKind::Custom(MY_CUSTOM_TOAST),
                        options: self.toast_options.unwrap(),
                    });            
                }

                ui.separator();

                ui.label("Fornecedores: ");

                if ui.add_sized([80., 25.], egui::Button::new("Listar")).clicked() {
                    let suppliers = rt.block_on(self.retrieve_suppliers());
                    if suppliers.is_empty() {
                        println!("Nenhum fornecedor encontrado");
                        return;
                    }
                    suppliers.into_iter().for_each(|supplier: Fornecedor| {
                        println!("{:?}", supplier);
                    });
                }
            });
    }

    async fn insert_product(&mut self, file_path: String) -> Produto {
        let path = Path::new(&file_path);
        let filename = path.file_stem().and_then(OsStr::to_str).unwrap();
        let extension = path.extension().and_then(OsStr::to_str).unwrap();

        let prod_category: Option<i32> = None;
        let prod_supplier: Option<i32> = None;

        let inserted: Produto = sqlx::query_as!(Produto, 
            "INSERT INTO produto( nome, preco, categoria, fornecedor, descricao, foto, \"formatoImagem\", \"dataCriacao\" )
               VALUES ( $1, $2, $3, $4, $5, $6, $7, $8 )
               RETURNING id, nome, preco, categoria, fornecedor, descricao, foto, \"formatoImagem\" as formato_imagem, \"dataCriacao\" as data_criacao;",
               filename.to_string(),
               99.00,
               prod_category,
               prod_supplier,
               Some("Descrição do produto".to_string()),
               Some(file_to_base64(file_path.clone())),
               Some(format!("image/{};base64", extension)),
               Some(chrono::Local::now().naive_local())
            )
            .fetch_one(&self.pool)
            .await.expect("Unable to query database table");
            inserted
    
    }

    async fn retrieve_products(&mut self) -> Vec<Produto> {
        let products: Vec<Produto> = sqlx::query_as!(Produto, 
        "SELECT id,
                nome,
                preco,
                categoria,
                fornecedor,
                descricao,
                foto,
                \"formatoImagem\" as formato_imagem,
                \"dataCriacao\" as data_criacao
        FROM produto;")
            .fetch_all(&self.pool)
            .await.expect("Unable to query database table");
        products
    }

    async fn retrieve_categories(&mut self) -> Vec<Categoria> {
        let categories: Vec<Categoria> = sqlx::query_as!(Categoria, "select * from categoria")
            .fetch_all(&self.pool)
            .await.expect("Unable to query database table");
        categories
    }

    async fn retrieve_suppliers(&mut self) -> Vec<Fornecedor> {
        let suppliers: Vec<Fornecedor> = sqlx::query_as!(Fornecedor,"select * from fornecedor")
            .fetch_all(&self.pool)
            .await.expect("Unable to query database table");
        suppliers
    }
}

fn file_to_base64(file_path: String) -> String {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut file_data: Vec<u8> = Vec::new();
    file.read_to_end(&mut file_data).expect("Failed to read file data");
    let encoded: String = general_purpose::STANDARD.encode(file_data);
    encoded
}

// TODO: fix InvalidPadding           STANDARD_NO_PAD -> STANDARD
fn export_picture(product: Produto, output_dir: String) {
    let extension: String = product.formato_imagem.unwrap().replace("image/", "").replace(";base64", "");
    let file_path: String = format!("{}/{}.{}", output_dir, product.nome, extension);
    println!("Exporting picture: {}", file_path);

    let encoded  = product.foto.unwrap();
    let file_data = general_purpose::STANDARD.decode(encoded).unwrap_or_else(|e| {
        println!("Error: {}", e);
        Vec::new()
    });
    if file_data.is_empty() { return }

    let mut file = File::create(file_path).unwrap();
    file.write_all(&file_data).unwrap_or_else(|e| {
        println!("Error: {}", e);
        return
    });
    file.flush().unwrap_or_else(|e| {
        println!("Error: {}", e);
        return
    });
}

fn my_custom_toast_contents(ui: &mut egui::Ui, toast: &mut Toast) -> egui::Response {
    Frame::default()
        .fill(Color32::from_rgb(33, 150, 243))
        .inner_margin(Margin::same(12.0))
        .rounding(4.0)
        .show(ui, |ui| {
            ui.label(toast.text.clone().color(Color32::WHITE).monospace());

            if egui::Button::new(RichText::new("Close").color(Color32::WHITE))
                .fill(Color32::from_rgb(33, 150, 243))
                .stroke((1.0, Color32::WHITE))
                .ui(ui)
                .clicked()
            {
                toast.close();
            }
        })
        .response
}
