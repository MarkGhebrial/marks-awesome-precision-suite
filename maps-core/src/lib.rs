use opencv as cv;
use cv::core::Mat;
use cv::core::Vector;
use cv::core::Point;
use cv::imgcodecs;
use cv::imgproc;
use pipeline::stages::*;
use pipeline::Pipeline;
use pipeline::PipelineStage;

// mod cv_pipeline;
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

const THRESHOLD: f64 = 159.0;

// `Vector`` is the C++ vector type. It is different from Rust's `Vec` type
type Contour = Vector<Point>;

pub fn load_image() -> Mat {
    imgcodecs::imread_def(
        "/home/markg/Documents/Code/Marks-Awesome-Precision-Suite/images/testtarget15.jpg",
    )
    .expect("Could not find image.")
}

pub fn test_function() -> Mat {
    let mut image: Mat = load_image();

    let (_, contour) = find_target_corners(&image);

    // cv::imgproc::draw_contours_def(&mut image, &contour, -1, Scalar::from([0.0, 255.0, 0.0, 0.0])).unwrap();
    for i in 0..contour.len() {
        imgproc::line(
            &mut image,
            contour.get(i).unwrap(),
            contour.get((i + 1) % contour.len()).unwrap(),
            [0.0, 255.0, 0.0, 0.0].into(),
            5,
            imgproc::LINE_8,
            0,
        )
        .unwrap();
    }

    image
}

pub fn find_target_corners(image: &Mat) -> (Mat, Vector<Point>) {
    let mut pipeline: Pipeline = Pipeline::new();
    pipeline.add_stage(ConvertColorStage::rgba_to_grayscale());
    pipeline.add_stage(GaussianBlurStage::default());
    pipeline.add_stage(ThresholdStage::default().set_threshold(THRESHOLD));

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
    let mut biggest_contour: Contour = Vector::new();
    let mut area_of_biggest_contour = 0.0;
    for contour in contours {
        // Simplify the contour so that we know the number of vertices is correct
        let mut simplified_contour: Contour = Vector::new();
        let epsilon: f64 = 0.02 * imgproc::arc_length(&contour, true).unwrap();
        imgproc::approx_poly_dp(&contour, &mut simplified_contour, epsilon, true).unwrap();

        if simplified_contour.len() == 4 {
            let area = imgproc::contour_area_def(&simplified_contour).unwrap();

            if area > area_of_biggest_contour {
                area_of_biggest_contour = area;
                biggest_contour = simplified_contour;
            }
        }
    }

    (img_copy, biggest_contour)
}
