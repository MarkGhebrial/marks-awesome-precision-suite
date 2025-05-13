use opencv as cv;
use cv::core::no_array;
use cv::core::ToInputArray;
use cv::imgproc;
use cv::core::Mat;
use cv::core::Point;
use cv::core::Scalar;

use super::super::PipelineStage;

pub struct DrawContoursStage<T: ToInputArray> {
    contours: T,
    color: Scalar,
    thickness: i32,
}

impl<T: ToInputArray> DrawContoursStage<T> {
    pub fn new(contours: T, color: Scalar, thickness: i32) -> Self {
        Self {
            contours,
            color,
            thickness,
        }
    }
}

impl<T: ToInputArray> PipelineStage for DrawContoursStage<T> {
    fn compute(&self, image: &mut Mat) {
        // TODO: This fails an assertion somewhere in opencv:
        // OpenCV(4.11.0) /usr/src/debug/opencv/opencv/modules/core/src/matrix_wrap.cpp:43: error: (-215:Assertion failed) i < 0 in function 'getMat_'\n
        imgproc::draw_contours(
            image,
            &self.contours,
            -1,
            self.color,
            self.thickness,
            imgproc::LineTypes::LINE_8.into(),
            &no_array(),
            i32::MAX,
            Point::new(0,0)
        ).unwrap();
    }
}
