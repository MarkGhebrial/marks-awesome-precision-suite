use opencv::core::MatTrait;
use opencv::traits::Boxed;
use opencv as cv;
use cv::core::Mat;
use cv::core::Size;
use cv::imgproc;

use super::super::PipelineStage;

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
        // Create a second reference to the Mat without actually copying its data.
        let i = unsafe {
            // Increment the Mat's reference counter to avoid a double free
            image.addref().expect("unable to increment reference count for Mat");

            Mat::from_raw(image.as_raw_mut())
        };

        // Blur the image
        imgproc::gaussian_blur_def(
            &i,
            image,
            self.size,
            self.sigma_x,
            // 0.0,
            // BorderTypes::BORDER_REFLECT.into(),
            // AlgorithmHint::ALGO_HINT_DEFAULT,
        )
        .unwrap();
    }
}
