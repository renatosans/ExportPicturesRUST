use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;
use std::io::{Read, Write};
use tinyfiledialogs::*;
use base64::{Engine as _, engine::general_purpose};

use dotenv::dotenv;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use eframe::egui;
use egui::style::Margin;
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
use egui::{Color32, Direction, Frame, Pos2, RichText, Widget};

mod models;
mod schema;
use models::*;
use schema::produto::dsl::produto;
use std::time::Duration;


/// Identifier for a custom toast kind
const MY_CUSTOM_TOAST: u32 = 0;
// pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;


fn main() {
    eframe::run_native(
        "Export Pictures (Catalogo de produtos)",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(Demo::default())),
    ).unwrap();
}

struct Demo {
    duration_sec: f32,
    kind: ToastKind,
    show_icon: bool,
    pool: DbPool,
    toasts: Toasts,
    toast_options: Option<ToastOptions>
}

impl Default for Demo {
    fn default() -> Self {
        dotenv().ok();
        let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL");
        // let manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(database_url);
        let manager: ConnectionManager<MysqlConnection> = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .unwrap_or_else(|e| {
            println!("Error: {}", e);
            std::process::exit(0); // don´t panic
        });
        let toasts: Toasts = Toasts::new()
        .anchor(Pos2::new(50.0, 50.0))
        .direction(Direction::TopDown)
        .align_to_end(false)
        .custom_contents(MY_CUSTOM_TOAST, my_custom_toast_contents);

        Self {
            duration_sec: 6.5,
            kind: ToastKind::Info,
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

                egui::ComboBox::from_label("Kind")
                    .selected_text(format!("{:?}", self.kind))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.kind, ToastKind::Info, "Info");
                        ui.selectable_value(&mut self.kind, ToastKind::Warning, "Warning");
                        ui.selectable_value(&mut self.kind, ToastKind::Error, "Error");
                        ui.selectable_value(&mut self.kind, ToastKind::Success, "Success");
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

                if ui.button("Give me a custom toast").clicked() {
                    self.toasts.add(Toast {
                        text: format!("Hello, I am a custom toast. Kind: {:?}", self.kind).into(),
                        kind: ToastKind::Custom(MY_CUSTOM_TOAST),
                        options,
                    });
                }

                ui.separator();

                if ui.add_sized([80., 25.], egui::Button::new("Inserir")).clicked() {
                    self.insert_product()
                }

                if ui.add_sized([80., 25.], egui::Button::new("Recuperar")).clicked() {
                    self.retrieve_products()
                }

            });
    }

    fn insert_product(&mut self) {
        let mut conn = self.pool.get().unwrap(); // TODO: fix unwrap

        let file_path: String;
        match open_file_dialog("Selecione a foto do produto", "*.*", None) {
            Some(file) => {
                file_path = file;
            }
            None => return,
        }
        let foto: String = file_to_base64(file_path.clone());
        let path = Path::new(&file_path);
        let filename = path.file_stem().and_then(OsStr::to_str).unwrap();
        let extension = path.extension().and_then(OsStr::to_str).unwrap();

        let new_product = Produto {
            id: 0,
            nome: filename.to_string(),
            preco: 99.00,
            categoria: None,
            fornecedor: None,
            descricao: Some("Descrição do produto".to_string()),
            foto: Some(foto),
            formato_imagem: Some(format!("image/{};base64", extension)),
            data_criacao: Some(chrono::Local::now().naive_local()),
        };

        diesel::insert_into(produto).values(new_product).execute(&mut conn).unwrap_or_else(|e| {
            println!("Error: {}", e);
            std::process::exit(0); // don´t panic
        });

        self.toasts.add(Toast {
            text: "Registro inserido com sucesso no banco".into(),
            kind: ToastKind::Custom(MY_CUSTOM_TOAST),
            options: self.toast_options.unwrap(),
        });

    }

    fn retrieve_products(&mut self) {
        let mut conn = self.pool.get().unwrap(); // TODO: fix unwrap

        let db_result: Result<Vec<Produto>, diesel::result::Error> = produto.load::<Produto>(&mut conn);

        let output_dir = format!("{}{}", std::env::current_dir().unwrap().display(), "/img");
        std::fs::create_dir_all(output_dir.clone()).unwrap();

        let catalogo = db_result.unwrap();
        catalogo.into_iter().for_each(|product: Produto| export_picture(product, output_dir.clone()));

        self.toasts.add(Toast {
            text: format!("Arquivos salvos em {}", output_dir.clone()).into(),
            kind: ToastKind::Custom(MY_CUSTOM_TOAST),
            options: self.toast_options.unwrap(),
        });
    }
}

fn file_to_base64(file_path: String) -> String {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut file_data: Vec<u8> = Vec::new();
    file.read_to_end(&mut file_data).expect("Failed to read file data");
    let encoded: String = general_purpose::STANDARD_NO_PAD.encode(file_data);
    encoded
}

// TODO: fix InvalidPadding
fn export_picture(product: Produto, output_dir: String) {
    let extension: String = product.formato_imagem.unwrap().replace("image/", "").replace(";base64", "");
    let file_path: String = format!("{}/{}.{}", output_dir, product.nome, extension);
    println!("Exporting picture: {}", file_path);

    let encoded  = product.foto.unwrap();
    let file_data = general_purpose::STANDARD_NO_PAD.decode(encoded).unwrap_or_else(|e| {
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
