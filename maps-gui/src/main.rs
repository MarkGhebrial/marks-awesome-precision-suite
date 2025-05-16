#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::sync::mpsc::{self, Receiver, RecvError, Sender};
use std::thread;

use cv::core::Mat;
use opencv as cv;

use eframe::egui;

mod image_panel;
use image_panel::*;
mod settings_panel;
use settings_panel::*;

mod egui_mat_image;

use maps_core::parameters::MAPSPipelineParams;

fn main() {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_app_id("MAPS"), //.with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let (tx_1, rx_1) = mpsc::channel::<Vec<(String, Mat)>>();

    let (tx_2, rx_2) = mpsc::channel::<MAPSPipelineParams>();

    // Spawn the thread that'll handle the image processing nonsense
    thread::spawn(move || {
        loop {
            let params = match rx_2.recv() {
                Ok(p) => p,
                Err(RecvError) => break, // The channel has disconnected, so exit the loop and kill the thread
            };

            let mut out = Vec::new();

            let img0 = maps_core::load_image();

            let (img1, corners) = maps_core::find_target_corners(&img0);

            let img2 = maps_core::transform_image(&img0, corners);

            out.push(("Original image".into(), img0));
            out.push(("Thresholded image (pretransform)".into(), img1));
            out.push(("Transformed image".into(), img2));
            tx_1.send(out).unwrap();
        }

        println!("Image processor thread terminating");
    });

    eframe::run_native(
        "MAPS",
        options,
        Box::new(|_cc| {
            // This gives us egui's image loading support:
            // egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(MyApp::new(rx_1, tx_2)))
        }),
    )
    .unwrap();
}

struct MyApp {
    image_viewer_panel: ImageViewerPanel,
    settings_panel: SettingsPanel,
}

impl MyApp {
    fn new(recv: Receiver<Vec<(String, Mat)>>, send: Sender<MAPSPipelineParams>) -> Self {
        Self {
            image_viewer_panel: ImageViewerPanel::new(recv),
            settings_panel: SettingsPanel::new(send),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Draw bottom panel
        egui::TopBottomPanel::bottom("bottom panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Bottom panel");
                ui.label("This is some text. Lorem Ipsum amirite?");

                ui.allocate_space(ui.available_size());
            });

        // Draw right panel
        egui::SidePanel::right("right panel")
            .resizable(false)
            .show(ctx, |ui| {
                self.settings_panel.draw_ui(ui);
            });

        // Draw image viewer panel
        egui::CentralPanel::default().show(ctx, |ui| {
            // self.settings_panel.draw_ui(ui);
            self.image_viewer_panel.draw_ui(ui);
        });
    }
}
