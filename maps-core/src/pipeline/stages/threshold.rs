use cv::core::Mat;
use cv::imgproc;
use cv::imgproc::AdaptiveThresholdTypes;
use cv::imgproc::ThresholdTypes;
use opencv as cv;
use opencv::core::ModifyInplace;

use super::super::PipelineStage;

/// Wrapper for `cv::imgproc::threshold`
pub struct ThresholdStage {
    pub threshold: f64,
    pub threshold_type: ThresholdTypes,
}

impl ThresholdStage {
    /// TODO: do we actually want to consume self here? I think that's how the
    /// builder pattern usually works, but is the builder pattern the right
    /// solution?
    pub fn set_threshold(mut self, thresh: f64) -> Self {
        self.threshold = thresh;
        self
    }

    pub fn set_threshold_type(mut self, thresh_type: ThresholdTypes) -> Self {
        self.threshold_type = thresh_type;
        self
    }
}

impl Default for ThresholdStage {
    fn default() -> Self {
        Self {
            threshold: 150.0,
            threshold_type: ThresholdTypes::THRESH_BINARY,
        }
    }
}

impl PipelineStage for ThresholdStage {
    fn compute(&self, image: &mut Mat) {
        // We have to do this unsafe `modify_inplace` nonsense because in-place opencv
        // operations require both an immutable reference and a mutable reference
        // to the same Mat. Safe Rust does not allow that.
        //
        // See https://github.com/twistedfall/opencv-rust/issues/571
        unsafe {
            image.modify_inplace(|input, output| {
                // TODO: do we need to make the maxval parameter one of the struct members?
                imgproc::threshold(
                    input,
                    output,
                    self.threshold,
                    255.0,
                    self.threshold_type.into(),
                )
                .unwrap();
            });
        }
    }
}

/// Wrapper for `opencv::imgproc::adaptive_threshold`
pub struct AdaptiveThresholdStage {
    pub threshold_type: ThresholdTypes,
    pub adaptive_method: AdaptiveThresholdTypes,
    pub block_size: i32,
    pub c: f64,
}

impl AdaptiveThresholdStage {
    pub fn set_threshold_type(mut self, thresh_type: ThresholdTypes) -> Self {
        self.threshold_type = thresh_type;
        self
    }

    pub fn set_adaptive_method(mut self, method: AdaptiveThresholdTypes) -> Self {
        self.adaptive_method = method;
        self
    }

    pub fn set_block_size(mut self, block_size: i32) -> Self {
        self.block_size = block_size;
        self
    }

    pub fn set_c(mut self, c: f64) -> Self {
        self.c = c;
        self
    }
}

impl Default for AdaptiveThresholdStage {
    fn default() -> Self {
        Self {
            threshold_type: ThresholdTypes::THRESH_BINARY,
            adaptive_method: AdaptiveThresholdTypes::ADAPTIVE_THRESH_GAUSSIAN_C,
            block_size: 3,
            c: 0.0, // TODO: Look up what this parameter does and set a sensible default value
        }
    }
}

impl PipelineStage for AdaptiveThresholdStage {
    fn compute(&self, image: &mut Mat) {
        // We have to do this unsafe `modify_inplace` nonsense because in-place opencv
        // operations require both an immutable reference and a mutable reference
        // to the same Mat. Safe Rust does not allow that.
        //
        // See https://github.com/twistedfall/opencv-rust/issues/571
        unsafe {
            image.modify_inplace(|input, output| {
                imgproc::adaptive_threshold(
                    input,
                    output,
                    255.0,
                    self.adaptive_method.into(),
                    self.threshold_type.into(),
                    self.block_size,
                    self.c,
                )
                .unwrap();
            })
        }
    }
}
