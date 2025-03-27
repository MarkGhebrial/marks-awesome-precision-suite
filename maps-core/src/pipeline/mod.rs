use cv::core::Mat;
use opencv as cv;

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

/// Takes an image as input and returns some other type.
pub trait FinalPipelineStage {
    type Output;

    fn compute(&self, image: &mut Mat) -> Self::Output;

    fn compute_on_a_copy(&self, image: &Mat) -> (Mat, Self::Output) {
        let mut output_image = image.clone();

        let output_data = self.compute(&mut output_image);

        (output_image, output_data)
    }
}

impl<T> PipelineStage for dyn FinalPipelineStage<Output = T> {
    fn compute(&self, image: &mut Mat) {
        self.compute(image); // Do the computations on the image, but don't return the extra data
    }
}
