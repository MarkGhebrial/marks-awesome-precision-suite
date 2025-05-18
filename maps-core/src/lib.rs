use cv::core::Mat;
use cv::core::Point;
use cv::core::Vector;
use cv::imgcodecs;
use cv::imgproc;
use opencv as cv;
use opencv::core::Scalar;
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

// TODO: Delete this constant
const THRESHOLD: f64 = 159.0;

// `Vector` is the C++ vector type. It is different from Rust's `Vec` type
type Contour = Vector<Point>;

pub fn load_image() -> Mat {
    imgcodecs::imread_def(
        "/home/markg/Documents/Code/Marks-Awesome-Precision-Suite/images/testtarget15.jpg",
    )
    .expect("Could not find image.")
}

// pub fn test_function() -> Mat {
//     let mut image: Mat = load_image();

//     let (_, contour) = find_target_corners(&image);

//     // cv::imgproc::draw_contours_def(&mut image, &contour, -1, Scalar::from([0.0, 255.0, 0.0, 0.0])).unwrap();
//     for i in 0..contour.len() {
//         imgproc::line(
//             &mut image,
//             contour.get(i).unwrap(),
//             contour.get((i + 1) % contour.len()).unwrap(),
//             [0.0, 255.0, 0.0, 0.0].into(),
//             5,
//             imgproc::LINE_8,
//             0,
//         )
//         .unwrap();
//     }

//     image
// }

pub fn find_target_corners(image: &Mat, threshold_method: ThresholdMode) -> (Mat, Vec<Point>) {
    let pipeline = ConvertColorStage::rgba_to_grayscale()
        .chain(GaussianBlurStage::default())
        .dyn_chain(match threshold_method {
            ThresholdMode::Manual { thresh } => {
                Box::new(ThresholdStage::default().set_threshold(thresh))
            }
            ThresholdMode::Otsu => {
                println!("TODO: Implement Otsu thresholding");
                Box::new(ThresholdStage::default().set_threshold(THRESHOLD))
            }
            ThresholdMode::Automatic { c } => {
                Box::new(AdaptiveThresholdStage::default().set_adaptive_method(
                    imgproc::AdaptiveThresholdTypes::ADAPTIVE_THRESH_GAUSSIAN_C,
                ))
            }
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

pub fn transform_image(image: &Mat, corners: Vec<Point>) -> Mat {
    TransformStage::new(corners[0..4].try_into().unwrap()).compute_on_a_copy(image)
}
