use opencv as cv;
use cv::core::Mat;
use cv::core::MatTraitConst;
use cv::core::Size;
use cv::core::VecN;
use cv::imgproc;

pub trait PipelineStage {
    fn compute(&self, image: Mat) -> Mat;
}

pub struct GaussianBlurStage {
    size: Size,
    sigma_x: f64,
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
    fn compute(&self, image: Mat) -> Mat {
        let mut out = Mat::new_size_with_default(
            image.size().unwrap(),
            image.typ(),
            VecN::new(0.0, 0.0, 0.0, 0.0),
        )
        .unwrap();

        // Blur the image
        imgproc::gaussian_blur_def(
            &image,
            &mut out,
            self.size,
            self.sigma_x,
            // 0.0,
            // BorderTypes::BORDER_REFLECT.into(),
            // AlgorithmHint::ALGO_HINT_DEFAULT,
        )
        .unwrap();

        out
    }
}
