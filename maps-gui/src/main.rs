#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui::{self, Spacing};

mod imagepanel;
use imagepanel::*;

mod egui_mat_image;

fn main() {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_app_id("MAPS"), //.with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "MAPS",
        options,
        Box::new(|_cc| {
            // This gives us image support:
            // egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
    .unwrap();

    println!("Hi, this code only runs after the GUI terminates. That makes sense.");
}

struct MyApp {
    name: String,
    age: u32,
    spacing: f32,
    is_left_panel_expanded: bool,
    image_viewer_panel: ImageViewerPanel,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            spacing: Spacing::default().item_spacing.y,
            is_left_panel_expanded: true,
            image_viewer_panel: ImageViewerPanel::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ctx.tex_manager();

        egui::SidePanel::left("image viewer panel")
            .resizable(true)
            .show_animated(ctx, self.is_left_panel_expanded, |ui| {
                self.image_viewer_panel.draw_ui(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.heading("My egui Application");
                    ui.horizontal(|ui| {
                        let name_label = ui.label("Your name: ");
                        ui.text_edit_singleline(&mut self.name)
                            .labelled_by(name_label.id);
                    });
                    ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
                    if ui.button("Increment").clicked() {
                        self.age += 1;
                    }
                    ui.label(format!("Hello '{}', age {}", self.name, self.age));
                });

                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = self.spacing;

                    ui.heading("Group heading");

                    ui.label("I have no idea what to put here");

                    if ui.button("Button that does nothing :)").clicked() {
                        println!("Wow, button was clicked");

                        self.is_left_panel_expanded = !self.is_left_panel_expanded;

                        if self.spacing == Spacing::default().item_spacing.y {
                            self.spacing = 100.0;
                        } else {
                            self.spacing = Spacing::default().item_spacing.y;
                        }
                    };
                });
            });
        });
    }
}
