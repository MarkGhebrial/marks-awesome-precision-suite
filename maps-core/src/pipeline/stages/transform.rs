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
    /// Width to height aspect ratio. Enforced by `.width()` and `.height()` if
    /// Some.
    aspect_ratio: Option<f64>,

    corners: [Point; 4],
}

impl TransformStage {
    /// Default width, height: 500, 500
    pub fn new(corners: [Point; 4]) -> Self {
        Self {
            height: 500, // TODO: change the default height and width
            width: 500,
            aspect_ratio: None,
            corners,
        }
    }

    /// Set the width/height aspect ratio. The ratio will be enforced in
    /// subsequent calls to `.width()` and `.height()`.
    pub fn aspect_ratio(mut self, width_to_height: f64) -> Self {
        self.aspect_ratio = Some(width_to_height);
        self
    }

    pub fn width(mut self, width: i32) -> Self {
        self.width = width;

        // If `self.aspect_ratio` is `Some`, then enforce that ratio
        if let Some(ratio) = self.aspect_ratio {
            self.height = (self.width as f64 / ratio + 0.5) as i32;
        }
        self.enforce_max_dimensions();

        self
    }

    pub fn height(mut self, height: i32) -> Self {
        self.height = height;

        // If `self.aspect_ratio` is `Some`, then enforce that ratio
        if let Some(ratio) = self.aspect_ratio {
            self.width = (self.height as f64 + ratio + 0.5) as i32;
        }
        self.enforce_max_dimensions();

        self
    }

    /// Make sure the number of pixels in the output image is not too large.
    /// Eframe asserts that neither the width nor height is more than 16384
    /// pixels.
    fn enforce_max_dimensions(&mut self) {
        if self.width > 16384 {
            self.width = 16384;
        }
        if self.height > 16384 {
            self.height = 16384;
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
