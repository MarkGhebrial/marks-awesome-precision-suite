use cv::core::{Mat, Point, Vector};
use cv::imgproc::adaptive_threshold;
use opencv as cv;
use opencv::core::MatTraitConst;

pub enum ThresholdMode {
    /// Adaptive thresholding.
    AUTOMATIC { c: f64 },

    /// Binary thresholding using the Otsu algorithm to calculate the threshold.
    OTSU,

    /// Binary thresholding using a user-specified threshold.
    MANUAL { thresh: f64 },
}

pub struct MAPSPipelineParams {
    /// Dimension of the target
    pub target_dimensions: (f64, f64),

    /// Thresholding mode for the first thresholding step
    pub corner_thresh_mode: ThresholdMode,

    /// Thresholding mode for the second thresholding step
    pub dot_thresh_mode: ThresholdMode,

    /// Height in pixels of the transformed image
    pub transformed_image_size: usize,
}

impl Default for MAPSPipelineParams {
    fn default() -> Self {
        Self {
            target_dimensions: (8.5, 11.0),
            corner_thresh_mode: ThresholdMode::AUTOMATIC { c: 2.0 },
            dot_thresh_mode: ThresholdMode::AUTOMATIC { c: 2.0 },
            transformed_image_size: 1500,
        }
    }
}

pub struct MAPSPipeline {
    input_image: Mat,
    params: MAPSPipelineParams,
}

impl MAPSPipeline {
    pub fn new(image: Mat) -> Self {
        Self {
            input_image: image,
            params: MAPSPipelineParams::default(),
        }
    }

    pub fn first_stage(&self) {

        // adaptive_threshold(&self.input_image.clone(), &mut self.input_image, max_value, adaptive_method, threshold_type, block_size, c)
    }
}
