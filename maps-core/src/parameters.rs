#[derive(Debug, PartialEq, Clone)]
pub struct MAPSPipelineParams {
    /// File path for the image to load
    pub image_path: String,

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
            image_path: "".into(),
            target_dimensions: (8.5, 11.0),
            corner_thresh_mode: ThresholdMode::Automatic { c: 2.0 },
            dot_thresh_mode: ThresholdMode::Automatic { c: 2.0 },
            transformed_image_size: 1500,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ThresholdMode {
    /// Adaptive thresholding.
    Automatic { c: f64 },

    /// Binary thresholding using the Otsu algorithm to calculate the threshold.
    Otsu,

    /// Binary thresholding using a user-specified threshold.
    Manual { thresh: f64 },
}

// #[derive(Debug, PartialEq, Copy, Clone)]
// pub enum CornerLocationMode {
//     /// Use automatic thresholding to find the corners
//     Automatic,

//     /// Use a user-specified threshold
//     ManualThresh { thresh: f64 },
// }