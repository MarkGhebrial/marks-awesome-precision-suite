// TODO: This is from the egui template. Do we need it?
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::sync::mpsc::{self, RecvError};
use std::thread;

use cv::core::Mat;
use opencv as cv;

use eframe::egui;

mod app;
use app::MyApp;

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

            let (img1, corners) = maps_core::find_target_corners(&img0, params.corner_thresh_mode);

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
