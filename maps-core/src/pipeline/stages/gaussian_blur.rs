use cv::core::Mat;
use cv::core::Size;
use cv::imgproc;
use opencv as cv;
use opencv::core::ModifyInplace;

use crate::pipeline::PipelineStage;

pub struct GaussianBlurStage {
    pub size: Size,
    pub sigma_x: f64,
}

impl Default for GaussianBlurStage {
    fn default() -> Self {
        Self {
            size: Size::new(15, 15),
            sigma_x: 0.0,
        }
    }
}

impl PipelineStage for GaussianBlurStage {
    fn compute(&self, image: &mut Mat) {
        // We have to do this unsafe `modify_inplace` nonsense because using the Gaussian
        // blur function without copying all the Mat's data requires both an immutable reference and a mutable reference
        // to the same Mat. Safe Rust does not allow that.
        //
        // See https://github.com/twistedfall/opencv-rust/issues/571
        unsafe {
            image.modify_inplace(|input, output| {
                // Blur the image
                imgproc::gaussian_blur_def(input, output, self.size, self.sigma_x).unwrap();
            });
        }
    }
}
