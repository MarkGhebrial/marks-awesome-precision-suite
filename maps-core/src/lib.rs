use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

use cv::core::Mat;
use cv::core::Point;
use cv::core::Vector;
use cv::imgcodecs;
use cv::imgproc;
use imgproc::ThresholdTypes;
use opencv as cv;
use opencv::core::Scalar;
use opencv::core::VecN;
use parameters::ThresholdMode;
use pipeline::stages::*;
use pipeline::PipelineStage;

pub mod pipeline;

pub mod parameters;

// Pipeline stages:
//  1. Threshold the image
//      - Params:
//         - thresholding mode:
//           1: automatic (adaptive threshold)
//           2: Otsu (automatic binary threshold)
//           3: Manual (binary threshold with manually set value)
//  2. Find the corners of the target and transform the image
//  3. Threshold the transformed image and find the locations of all the marks
//      - Params: thresholding mode and
//  4. Draw

// `Vector` is the C++ vector type. It is different from Rust's `Vec` type
type Contour = Vector<Point>;

pub fn load_image() -> Mat {
    imgcodecs::imread_def(
        "/home/markg/Documents/Code/Marks-Awesome-Precision-Suite/images/testtarget15.jpg",
    )
    .expect("Could not find image.")
}

pub fn find_target_corners(image: &Mat, threshold_method: ThresholdMode) -> (Mat, Vec<Point>) {
    let pipeline = ConvertColorStage::rgba_to_grayscale()
        .chain(GaussianBlurStage::default())
        .dyn_chain(match threshold_method {
            ThresholdMode::Manual { thresh } => {
                Box::new(ThresholdStage::default().set_threshold(thresh))
            }
            ThresholdMode::Otsu => {
                Box::new(ThresholdStage::default().set_threshold_type(ThresholdTypes::THRESH_OTSU))
            }
            ThresholdMode::Adaptive { block_size, c } => Box::new(
                AdaptiveThresholdStage::default()
                    .set_adaptive_method(
                        imgproc::AdaptiveThresholdTypes::ADAPTIVE_THRESH_GAUSSIAN_C,
                    )
                    .set_block_size(block_size)
                    .set_c(c),
            ),
        });

    let mut img_copy = pipeline.compute_on_a_copy(image);

    // Step two: find the contours
    let mut contours: Vector<Contour> = Vector::new();
    imgproc::find_contours_def(
        &img_copy,
        &mut contours,
        imgproc::RETR_CCOMP,
        imgproc::CHAIN_APPROX_SIMPLE,
    )
    .unwrap();

    // Convert back to rgb so we can draw colored lines
    ConvertColorStage::grayscale_to_rgba().compute(&mut img_copy);

    // Step three: find the four-sided contour with the largest area
    let mut biggest_contour: Vec<Point> = Vec::new();
    let mut area_of_biggest_contour = 0.0;
    for contour in contours.iter() {
        // Simplify the contour so that we know the number of vertices is correct
        let mut simplified_contour: Contour = Vector::new();
        let epsilon: f64 = 0.02 * imgproc::arc_length(&contour, true).unwrap();
        imgproc::approx_poly_dp(&contour, &mut simplified_contour, epsilon, true).unwrap();

        if simplified_contour.len() == 4 {
            let area = imgproc::contour_area_def(&simplified_contour).unwrap();

            if area > area_of_biggest_contour {
                area_of_biggest_contour = area;
                biggest_contour = simplified_contour.into();
            }
        }
    }

    // Draw markers on the four corners
    DrawMarkerStage::new(
        biggest_contour.clone(),
        Scalar::from_array([255.0, 90.0, 0.0, 0.0]),
        15,
    )
    .marker_type(imgproc::MarkerTypes::MARKER_TILTED_CROSS)
    .marker_size(150)
    .compute(&mut img_copy);

    (img_copy, biggest_contour)
}

/// TODO: Is an entire new error type really needed here?
#[derive(Debug)]
pub struct TransformError;

impl Display for TransformError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("wrong number of points")
    }
}

impl Error for TransformError {}

pub fn transform_image(
    image: &Mat,
    corners: Vec<Point>,
    width: f64,
    height: f64,
) -> Result<Mat, TransformError> {
    if corners.len() != 4 {
        return Err(TransformError);
    }

    Ok(TransformStage::new(corners[0..4].try_into().unwrap())
        .aspect_ratio(width / height)
        .width(1500)
        .compute_on_a_copy(image))
}

/// Given an image of a bright target with dark dots on it, find the locations of the dots.
///
/// TODO: Change return type to `Contour` type alias
pub fn find_dots(image: &Mat) -> (Mat, Vec<Point>) {
    // Threshold the image. Probably with an adaptive threshold
    let thresholded_image = ConvertColorStage::rgba_to_grayscale()
        .chain(GaussianBlurStage::default())
        .chain(
            AdaptiveThresholdStage::default()
                .set_threshold_type(ThresholdTypes::THRESH_BINARY_INV)
                .set_block_size(125)
                .set_c(5.0),
        ) //.set_adaptive_method(imgproc::AdaptiveThresholdTypes::ADAPTIVE_THRESH_GAUSSIAN_C))
        .chain(ErodeOrDilateStage::circular_kernel(
            ErodeOrDilateOperation::ErodeThenDilate,
            20,
        ))
        // .chain(ThresholdStage::default().set_threshold_type(ThresholdTypes::THRESH_OTSU))
        .compute_on_a_copy(&image);

    // Find blobs/contours
    let mut contours: Vector<Contour> = Vector::new();
    imgproc::find_contours_def(
        &thresholded_image,
        &mut contours,
        imgproc::RETR_CCOMP,
        imgproc::CHAIN_APPROX_SIMPLE,
    )
    .unwrap();

    for contour in &contours {
        println!("Contour has the following points:");
        for point in contour {
            println!("{} {}", point.x, point.y);
        }
    }

    // new_image is the one with contours drawn on it
    let new_image = DrawContoursStage::new(&contours, VecN::new(1.0, 1.0, 0.0, 0.0), 2)
        .compute_on_a_copy(image);

    (new_image, Vec::new())

    // Draw crosshairs on each dot
    // Draw bounding circle on image

    // Return the coordinates of the contours

    // todo!();
}
