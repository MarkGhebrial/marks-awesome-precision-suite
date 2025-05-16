// use maps_core::find_target_corners;
use opencv::core::Size;
// use opencv::core::{Scalar, VecN};
use opencv::highgui;
use opencv::highgui::{/*named_window_def,*/ WINDOW_NORMAL};
use opencv::imgcodecs;
use opencv::prelude::*;
// use opencv::viz::imshow_def;

use maps_core::pipeline::stages::*;
use maps_core::pipeline::*;

fn main() {
    let mut image: Mat = imgcodecs::imread_def(
        "/home/markg/Documents/Code/Marks-Awesome-Precision-Suite/images/testtarget15.jpg",
    )
    .expect("Could not find image.");

    let pipeline = GaussianBlurStage {
        size: Size::new(15, 15),
        sigma_x: 0.0,
    };

    pipeline.compute(&mut image);

    // let contour = find_target_corners(&image).1;

    // opencv::imgproc::draw_contours_def(&mut image, &contour, -1, Scalar::from([0.0, 255.0, 0.0, 0.0])).unwrap();
    // for i in 0..contour.len() {
    //     imgproc::line(
    //         &mut image,
    //         contour.get(i).unwrap(),
    //         contour.get((i + 1) % contour.len()).unwrap(),
    //         [0.0, 255.0, 0.0, 0.0].into(),
    //         5,
    //         imgproc::LINE_8,
    //         0,
    //     )
    //     .unwrap();
    // }

    highgui::named_window("winname", WINDOW_NORMAL).unwrap();
    highgui::imshow("winname", &image).unwrap();
    highgui::wait_key(0).unwrap();

    println!("Cadf");
}
