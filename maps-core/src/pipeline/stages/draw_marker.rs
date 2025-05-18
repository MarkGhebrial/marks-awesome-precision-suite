use cv::core::Mat;
use cv::core::Point;
use cv::core::Scalar;
use cv::imgproc;
use opencv as cv;
use opencv::imgproc::MarkerTypes;
use opencv::imgproc::LINE_8;

use crate::pipeline::PipelineStage;

/// Use `cv::imgproc::draw_marker` to draw markers onto an image.
pub struct DrawMarkerStage {
    points: Vec<Point>,
    color: Scalar,
    marker_type: MarkerTypes,
    marker_size: i32,
    thickness: i32,
    line_type: i32,
}

impl DrawMarkerStage {
    pub fn new(points: Vec<Point>, color: Scalar, thickness: i32) -> Self {
        Self {
            points,
            color,
            marker_type: MarkerTypes::MARKER_CROSS,
            marker_size: 32,
            thickness,
            line_type: LINE_8,
        }
    }

    pub fn marker_size(mut self, marker_size: i32) -> Self {
        self.marker_size = marker_size;
        self
    }

    pub fn marker_type(mut self, marker_type: MarkerTypes) -> Self {
        self.marker_type = marker_type;
        self
    }

    pub fn line_type(mut self, line_type: i32) -> Self {
        self.line_type = line_type;
        self
    }
}

impl PipelineStage for DrawMarkerStage {
    fn compute(&self, image: &mut Mat) {
        for point in self.points.iter() {
            imgproc::draw_marker(
                image,
                *point,
                self.color,
                self.marker_type.into(),
                self.marker_size,
                self.thickness,
                self.line_type,
            )
            .unwrap();
        }
    }
}
