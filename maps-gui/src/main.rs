// TODO: This is from the egui template. Do we need it?
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::sync::mpsc::{self, RecvError, TryRecvError};
use std::sync::Arc;
use std::thread;

use cv::core::Mat;
use maps_core::pipeline::stages::ConvertColorStage;
use maps_core::pipeline::PipelineStage;
use opencv as cv;
use opencv::imgproc::ColorConversionCodes;

use eframe::egui;
use egui::Context;

mod app;
use app::MyApp;

mod util;

mod threshold_settings_panel;

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

    let (tx_1, rx_1) = mpsc::channel::<Vec<(String, Arc<Mat>)>>();

    let (tx_2, rx_2) = mpsc::channel::<(Context, MAPSPipelineParams)>();

    // Spawn the thread that'll handle the image processing nonsense
    thread::spawn(move || {
        'outer_loop: loop {
            // Fetch the egui context and pipeline parameters from the main thread
            let (mut ctx, mut params) = match rx_2.recv() {
                Ok(p) => p,
                Err(RecvError) => break 'outer_loop, // The channel has disconnected, so exit the loop and kill the thread
            };

            // Keep polling the channel until the last message is received
            'recv_loop: loop {
                match rx_2.try_recv() {
                    Ok((c, p)) => {
                        ctx = c;
                        params = p;
                    }
                    Err(TryRecvError::Empty) => break 'recv_loop,
                    Err(TryRecvError::Disconnected) => break 'outer_loop,
                }
            }

            // TODO: Move all this image processing sequence into a single function in the maps-core crate
            let mut img0 = maps_core::load_image();

            let (mut img1, corners) =
                maps_core::find_target_corners(&img0, params.corner_thresh_mode);

            let img2 = maps_core::transform_image(
                &img0,
                corners,
                params.target_dimensions.0,
                params.target_dimensions.1,
            );
            let mut img2 = match img2 {
                Ok(img) => img,
                Err(_) => {
                    println!("TRANSFORM FAILED!");
                    img0.clone()
                }
            };

            let (mut img3, _points) = maps_core::find_dots(&img2);

            // Do these actually convert to srgb instead of linear rgba? I have no idea :)
            let rgb_to_srgb = ConvertColorStage::new(ColorConversionCodes::COLOR_BGR2RGBA);
            let gray_to_srgb = ConvertColorStage::new(ColorConversionCodes::COLOR_BGR2RGBA);

            // Convert all the images to srgb so that egui can render them faster
            rgb_to_srgb.compute(&mut img0);
            gray_to_srgb.compute(&mut img1);
            rgb_to_srgb.compute(&mut img2);
            rgb_to_srgb.compute(&mut img3);

            let mut out = Vec::new();
            out.push(("Original image".into(), Arc::new(img0)));
            out.push(("Thresholded image (pretransform)".into(), Arc::new(img1)));
            out.push(("Transformed image".into(), Arc::new(img2)));
            out.push(("Final image".into(), Arc::new(img3)));
            tx_1.send(out).unwrap();

            ctx.request_repaint();
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
