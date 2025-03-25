use opencv as cv;
use cv::core::Mat;

pub mod stages;
// pub use sta::*;

mod pipeline;
pub use pipeline::*;

/// Represents a stage in a CV pipeline. Each stage takes an image as input and 
pub trait PipelineStage {
    /// Perform the computations on the provided image.
    fn compute(&self, image: &mut Mat);

    /// Perform the computations on a copy of the image, then return that copy.
    fn compute_on_a_copy(&self, image: &Mat) -> Mat {
        let mut out = image.clone();

        self.compute(&mut out);

        out
    }
}