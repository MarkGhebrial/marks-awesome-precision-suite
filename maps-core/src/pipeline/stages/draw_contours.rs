use opencv as cv;
use cv::imgproc;
// use cv::imgproc::LineTypes;
use cv::core::Mat;
use cv::core::Point;
use cv::core::Vec2f;
use cv::core::Vector;
use cv::core::Scalar;

use super::super::PipelineStage;

pub struct DrawContoursStage {
    contours: Vector<cv::core::Vec2f>,
    color: Scalar,
    thickness: i32,
}

impl DrawContoursStage {
    fn new(contours: Vector<Vec2f>, color: Scalar, thickness: i32) -> Self {
        Self {
            contours,
            color,
            thickness,
        }
    }
}

impl PipelineStage for DrawContoursStage {
    fn compute(&self, image: &mut Mat) {
        imgproc::draw_contours(
            image,
            &self.contours,
            -1,
            self.color,
            self.thickness,
            imgproc::LineTypes::LINE_8.into(),
            &[],
            i32::MAX,
            Point::new(0,0)
        ).unwrap();
    }
}
