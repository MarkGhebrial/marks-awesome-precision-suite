use cv::prelude::*;
use opencv as cv;

/// Represents a stage in an image processing pipeline. Each stage takes an
/// image as input and modifies that image in some way before the image is
/// passed to the next stage.
pub trait PipelineStage {
    /// Perform the computations in place on the provided image.
    fn compute(&self, image: &mut Mat);

    /// Perform the computations on a copy of the image, then return that copy.
    fn compute_on_a_copy(&self, image: &Mat) -> Mat {
        let mut out = image.clone();

        self.compute(&mut out);

        out
    }

    /// Pipe the output of this stage directly into the input of another stage.
    ///
    /// # Example (TODO: Finish example)
    /// ```rust
    /// use crate::pipeline::stages::*;
    ///
    /// let pipeline = ConvertColorStage::rgba_to_grayscale().chain(GaussianBlurStage::default());
    ///
    /// // Converts the image to grayscale, then performs a Gaussian blur
    /// pipeline.compute(&mut some_mat);
    /// ```
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

    /// Pipe the output of this stage directly into the input of a
    /// [`PipeLineStage`] trait object.
    ///
    /// Use this function when you don't know the type of the stage ahead of
    /// time.
    ///
    /// # Example
    /// ```rust
    /// let adaptive = true;
    ///
    /// let pipeline = ConvertColorStage::rgba_to_grayscale().
    ///     .dyn_chain(
    ///         if adaptive {
    ///             Box::new(ThresholdStage::default())
    ///         }
    ///         else {
    ///             Box::new(AdaptiveThresholdStage::default())
    ///         }
    ///     );
    /// ```
    fn dyn_chain(self, stage: Box<dyn PipelineStage>) -> DynChainedPipeline<Self>
    where
        Self: Sized,
    {
        DynChainedPipeline {
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

pub struct DynChainedPipeline<S1>
where
    S1: PipelineStage,
{
    stage1: S1,
    stage2: Box<dyn PipelineStage>,
}

impl<S1> PipelineStage for DynChainedPipeline<S1>
where
    S1: PipelineStage,
{
    fn compute(&self, image: &mut Mat) {
        self.stage1.compute(image);
        self.stage2.compute(image);
    }
}

/// `PipelineStage` implementation for functions that take a `&mut Mat`.
///
/// TODO: We're not using this anywhere. Delete it?
impl<T> PipelineStage for T
where
    T: Fn(&mut Mat),
{
    fn compute(&self, image: &mut Mat) {
        self(image)
    }
}
