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
pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;


fn main() {
    eframe::run_native(
        "Catalogo de produtos",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(Demo::default())),
    ).unwrap();
}

struct Demo {
    i: usize,
    anchor: Pos2,
    duration_sec: f32,
    direction: Direction,
    align_to_end: bool,
    kind: ToastKind,
    show_icon: bool,
    pool: DbPool,
}

impl Default for Demo {
    fn default() -> Self {
        dotenv().ok();
        let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL");
        let manager: ConnectionManager<MysqlConnection> = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .unwrap_or_else(|e| {
            println!("Error: {}", e);
            std::process::exit(0); // don´t panic
        });

        Self {
            i: 0,
            duration_sec: 2.0,
            anchor: Pos2::new(10.0, 10.0),
            direction: Direction::TopDown,
            align_to_end: false,
            kind: ToastKind::Info,
            show_icon: true,
            pool: pool
        }
    }
}

impl eframe::App for Demo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut toasts = Toasts::new()
            .anchor(self.anchor)
            .direction(self.direction)
            .align_to_end(self.align_to_end)
            .custom_contents(MY_CUSTOM_TOAST, my_custom_toast_contents);

        self.options_window(ctx, &mut toasts);

        toasts.show(ctx);

        ctx.request_repaint();
    }
}

impl Demo {
    fn options_window(&mut self, ctx: &egui::Context, toasts: &mut Toasts) {
        egui::Window::new("")
            .default_pos((100.0, 100.0))
            .default_width(200.0)
            .show(ctx, |ui| {

                egui::ComboBox::from_label("Direction")
                    .selected_text(format!("{:?}", self.direction))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.direction, Direction::TopDown, "TopDown");
                        ui.selectable_value(&mut self.direction, Direction::BottomUp, "BottomUp");
                        ui.selectable_value(
                            &mut self.direction,
                            Direction::RightToLeft,
                            "RightToLeft",
                        );
                        ui.selectable_value(
                            &mut self.direction,
                            Direction::LeftToRight,
                            "LeftToRight",
                        );
                    });

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

                if ui.button("Give me a custom toast").clicked() {
                    toasts.add(Toast {
                        text: format!("Hello, I am a custom toast {}", self.i).into(),
                        kind: ToastKind::Custom(MY_CUSTOM_TOAST),
                        options,
                    });

                    self.i += 1;
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

        let new_product = Produto {
            id: 0,
            nome: "Bola de futebol americano".to_string(),
            preco: 99.00,
            categoria: None,
            fornecedor: None,
            descricao: Some("Bola de futebol americano".to_string()),
            foto: None,
            formatoImagem: None,
            dataCriacao: Some(chrono::Local::now().naive_local()),
        };

        diesel::insert_into(produto).values(new_product).execute(&mut conn);
    }

    fn retrieve_products(&mut self) {
        let mut conn = self.pool.get().unwrap(); // TODO: fix unwrap

        let result: Result<Vec<Produto>, diesel::result::Error> = produto.load::<Produto>(&mut conn);
        println!("The result is {:#?}", result);
    }
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
