// use opencv::core::AlgorithmHint;
// use opencv::core::BorderTypes;
use opencv::core::Mat;
use opencv::core::Size;
use opencv::core::Vector;
use opencv::core::{Point, Point2f};
use opencv::imgcodecs;
use opencv::imgproc;

// mod cv_pipeline;
pub mod pipeline;

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
// type Contour2f = Vector<Point2f>;

pub fn load_image() -> Mat {
    imgcodecs::imread_def(
        "/home/markg/Documents/Code/Marks-Awesome-Precision-Suite/images/testtarget15.jpg",
    )
    .expect("Could not find image.")
}

pub fn test_function() -> Mat {
    let mut image: Mat = load_image();

    let (_, contour) = find_target_corners(&image);

    // opencv::imgproc::draw_contours_def(&mut image, &contour, -1, Scalar::from([0.0, 255.0, 0.0, 0.0])).unwrap();
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
    let mut img_copy = image.clone();

    // Step one: threshold the image
    // TODO?: Downscale the image?
    // Convert to grayscale (cv::cvtColor())
    imgproc::cvt_color_def(
        image,
        &mut img_copy,
        imgproc::ColorConversionCodes::COLOR_BGR2GRAY.into(),
    )
    .unwrap();

    // Blur the image
    imgproc::gaussian_blur_def(
        &img_copy.clone(),
        &mut img_copy,
        Size::new(15, 15),
        0.0,
        // 0.0,
        // BorderTypes::BORDER_REFLECT.into(),
        // AlgorithmHint::ALGO_HINT_DEFAULT,
    )
    .unwrap();

    // Threshold (cv::threshold())
    imgproc::threshold(
        &img_copy.clone(),
        &mut img_copy,
        THRESHOLD,
        255.0,
        imgproc::ThresholdTypes::THRESH_BINARY.into(),
    )
    .unwrap();



    // Step two: find the contours
    let mut contours: Vector<Contour> = Vector::new();
    imgproc::find_contours_def(
        &img_copy,
        &mut contours,
        imgproc::RETR_CCOMP,
        imgproc::CHAIN_APPROX_SIMPLE,
    )
    .unwrap();

    // Convert back to rgb
    imgproc::cvt_color_def(
        image,
        &mut img_copy,
        imgproc::ColorConversionCodes::COLOR_BGR2GRAY.into(), // TODO: this is suppoesed to be be COLOR_GRAY2BGR?
    )
    .unwrap();

    let output_mat = img_copy.clone();

    // Step three: find the four-sided contour with the largest area
    let mut biggest_contour: Contour = Vector::new();
    let mut area_of_biggest_contour = 0.0;
    for contour in contours {
        // Simplify the contour so that we know the number of verticies is correct
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

    (output_mat, biggest_contour)
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
