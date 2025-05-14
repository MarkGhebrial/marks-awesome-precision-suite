use cv::core::Mat;
use opencv as cv;

pub mod stages;
// pub use sta::*;

mod pipeline;
pub use pipeline::*;

/// Represents a stage in a CV pipeline. Each stage takes an image as input and
/// modifies that image in some way before the image is passed to the next stage.
pub trait PipelineStage {
    /// Perform the computations on the provided image.
    fn compute(&self, image: &mut Mat);

    /// Perform the computations on a copy of the image, then return that copy.
    fn compute_on_a_copy(&self, image: &Mat) -> Mat {
        let mut out = image.clone();

        self.compute(&mut out);

        out
    }

    fn chain<S2>(self, stage: S2) -> ChainedPipeline<Self, S2>
    where
        Self: Sized,
        S2: PipelineStage,
    {
        ChainedPipeline {
            stage1: self,
            stage2: stage,
        }
    }
}

pub struct ChainedPipeline<S1, S2>
where
    S1: PipelineStage,
    S2: PipelineStage,
{
    stage1: S1,
    stage2: S2,
}

impl<S1, S2> PipelineStage for ChainedPipeline<S1, S2>
where
    S1: PipelineStage,
    S2: PipelineStage,
{
    fn compute(&self, image: &mut Mat) {
        self.stage1.compute(image);
        self.stage2.compute(image);
    }
}

/// `PipelineStage` implementation for functions that take a `&mut Mat`
impl<T> PipelineStage for T
where 
    T: Fn(&mut Mat),
{
    fn compute(&self, image: &mut Mat) {
        self(image)
    }
}
