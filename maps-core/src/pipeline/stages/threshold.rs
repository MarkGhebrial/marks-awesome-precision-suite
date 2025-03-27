use cv::core::Mat;
use cv::imgproc;
use cv::imgproc::AdaptiveThresholdTypes;
use cv::imgproc::ThresholdTypes;
use opencv as cv;
use opencv::core::MatTrait;
use opencv::traits::Boxed;

use super::super::PipelineStage;

/// Wrapper for `cv::imgproc::ThresholdTypes`
pub struct ThresholdStage {
    pub threshold: f64,
    pub threshold_type: ThresholdTypes,
}

impl ThresholdStage {
    fn set_threshold(&mut self, thresh: f64) {
        self.threshold = thresh;
    }

    fn threshold_type(&mut self, thresh_type: ThresholdTypes) {
        self.threshold_type = thresh_type;
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
        // TODO: USE THE `modify_inplace` HELPER FUNCTION INSTEAD
        // Create a second reference to the Mat without actually copying its data.
        let i = unsafe {
            // Increment the Mat's reference counter to avoid a double free
            image
                .addref()
                .expect("unable to increment reference count for Mat");

            Mat::from_raw(image.as_raw_mut())
        };

        imgproc::threshold(
            &i,
            image,
            self.threshold,
            255.0, // TODO: do we need to make this parameter one of the struct members?
            self.threshold_type.into(),
        )
        .unwrap();
    }
}

/// Wrapper for `opencv::imgproc::adaptive_threshold`
pub struct AdaptiveThresholdStage {
    pub threshold: f64,
    pub threshold_type: ThresholdTypes,
    pub adaptive_method: AdaptiveThresholdTypes,
    pub block_size: i32,
    pub c: f64,
}

impl AdaptiveThresholdStage {
    fn set_threshold(&mut self, thresh: f64) {
        self.threshold = thresh;
    }

    fn threshold_type(&mut self, thresh_type: ThresholdTypes) {
        self.threshold_type = thresh_type;
    }
}

impl Default for AdaptiveThresholdStage {
    fn default() -> Self {
        Self {
            threshold: 150.0,
            threshold_type: ThresholdTypes::THRESH_BINARY,
            adaptive_method: AdaptiveThresholdTypes::ADAPTIVE_THRESH_GAUSSIAN_C,
            block_size: 3,
            c: 5.0, // TODO: Look up what this parameter does and set a sensible default value
        }
    }
}

impl PipelineStage for AdaptiveThresholdStage {
    fn compute(&self, image: &mut Mat) {
        // Create a second reference to the Mat without actually copying its data.
        let i = unsafe {
            // Increment the Mat's reference counter to avoid a double free
            image
                .addref()
                .expect("unable to increment reference count for Mat");

            Mat::from_raw(image.as_raw_mut())
        };

        imgproc::adaptive_threshold(
            &i,
            image,
            255.0,
            self.adaptive_method.into(),
            self.threshold_type.into(),
            self.block_size,
            self.c,
        )
        .unwrap();
    }
}
