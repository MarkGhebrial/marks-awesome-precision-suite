// TODO: This is from the egui template. Do we need it?
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::sync::mpsc::{self, RecvError};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use cv::core::Mat;
use opencv as cv;

use eframe::egui;
use egui::Context;

mod app;
use app::MyApp;

mod image_panel;
use image_panel::*;
mod settings_panel;
use settings_panel::*;

mod egui_mat_image;

use maps_core::parameters::MAPSPipelineParams;

type MatsSync = Arc<Mutex<Vec<(String, Arc<Mat>)>>>;
type ParamsSync = Arc<Mutex<MAPSPipelineParams>>;

fn main() {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_app_id("MAPS"), //.with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let params = Arc::new(Mutex::new(MAPSPipelineParams::default()));

    let mats: MatsSync = Arc::new(Mutex::new(Vec::new()));

    let params2 = params.clone();
    let mats2 = mats.clone();

    // let (tx_1, rx_1) = mpsc::channel::<Vec<(String, Mat)>>();

    // let (tx_2, rx_2) = mpsc::channel::<(Context, MAPSPipelineParams)>();

    // Spawn the thread that'll handle the image processing nonsense
    thread::spawn(move || {
        let mut last_params: MAPSPipelineParams = MAPSPipelineParams::default();

        loop {
            // let (ctx, params) = match rx_2.recv() {
            //     Ok(p) => p,
            //     Err(RecvError) => break, // The channel has disconnected, so exit the loop and kill the thread
            // };
            println!("Loop");

            let params = params.lock().expect("Failed to lock params mutex");
            if *params == last_params {
                println!("Continuing");
                drop(params);
                thread::sleep(Duration::from_millis(100));
                continue;
            }
            last_params = params.clone();

            let mut out = Vec::new();

            let img0 = maps_core::load_image();

            let (img1, corners) = maps_core::find_target_corners(&img0, params.corner_thresh_mode);

            let img2 = maps_core::transform_image(
                &img0,
                corners,
                params.target_dimensions.0,
                params.target_dimensions.1,
            );
            let img2 = match img2 {
                Ok(img) => img,
                Err(_) => {
                    println!("TRANSFORM FAILED!");
                    img0.clone()
                }
            };

            out.push(("Original image".into(), Arc::new(img0)));
            out.push(("Thresholded image (pretransform)".into(), Arc::new(img1)));
            out.push(("Transformed image".into(), Arc::new(img2)));
            let mut mats = mats.lock().expect("Failed to lock mats mutex");
            *mats = out;

            // tx_1.send(out).unwrap();

            // ctx.request_repaint();
        }

        // println!("Image processor thread terminating");
    });

    eframe::run_native(
        "MAPS",
        options,
        Box::new(|_cc| {
            // This gives us egui's image loading support:
            // egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(MyApp::new(mats2, params2)))
        }),
    )
    .unwrap();
}
