use cv::core::Mat;
use cv::core::ModifyInplace;
use cv::core::Point;
use cv::core::Point2f;
use cv::core::Vector;
use cv::imgproc;
use opencv as cv;

use crate::pipeline::PipelineStage;

/// Performs a perspective transform on the image.
pub struct TransformStage {
    height: i32,
    width: i32,
    corners: [Point; 4],
}

impl TransformStage {
    pub fn new(corners: [Point; 4]) -> Self {
        Self {
            height: 500, // TODO: change the default height and width
            width: 500,
            corners,
        }
    }
}

impl PipelineStage for TransformStage {
    fn compute(&self, image: &mut Mat) {
        let mut destination_points: Vector<Point2f> = Vector::with_capacity(4);
        destination_points.push((0.0, 0.0).into());
        destination_points.push((self.width as f32, 0.0).into());
        destination_points.push((self.width as f32, self.height as f32).into());
        destination_points.push((0.0, self.height as f32).into());

        // Make sure the corners are sorted in the right spatial order. We do this by
        // computing the "average location" of all the points and seeing which quadrant
        // each point lies in relative to that center.
        //
        // `center` is the point whose x and y values are the average x and y values
        // of `self.corners`
        let mut center: Point2f = Point2f::new(0.0, 0.0);
        for point in self.corners {
            center.x += point.x as f32;
            center.y += point.y as f32;
        }
        center.x /= 4.0;
        center.y /= 4.0;

        let mut sorted_corners: Vector<Point2f> = Vector::from_slice(&[Point2f::default(); 4]);
        for point in self.corners.map(|p| Point2f::new(p.x as f32, p.y as f32)) {
            let index = match (point.x > center.x, point.y > center.y) {
                (false, false) => 0, // Top left corner
                (true, false) => 1,  // Top right corner
                (true, true) => 2,   // Bottom right corner
                (false, true) => 3,  // Bottom left corner
            };
            sorted_corners.set(index, point).unwrap();
        }

        let transform =
            imgproc::get_perspective_transform_def(&sorted_corners, &destination_points).unwrap();

        // We have to do this unsafe `modify_inplace` nonsense because using the
        // warp_perspective function without cloning the Mat's data requires
        // both an immutable reference and a mutable reference to the same Mat.
        // Safe Rust does not allow that.
        //
        // See https://github.com/twistedfall/opencv-rust/issues/571
        unsafe {
            image.modify_inplace(|input, output| {
                imgproc::warp_perspective_def(
                    input,
                    output,
                    &transform,
                    (self.width, self.height).into(),
                )
                .unwrap();
            });
        }
    }
}
