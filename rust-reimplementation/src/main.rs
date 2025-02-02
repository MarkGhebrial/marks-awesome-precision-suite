use opencv::imgcodecs;
use opencv::prelude::*;
use rust_reimplementation::find_target_corners;

fn main() {
    let mut image: Mat = imgcodecs::imread_def(
        "/home/markg/Documents/Code/Marks-Awesome-Precision-Suite/images/testtarget15.jpg",
    )
    .expect("Could not find image.");

    let contour = find_target_corners(&image);

    opencv::imgproc::draw_contours_def(&mut image, &contour, contour_idx, color);

    println!("Cadf");
}
