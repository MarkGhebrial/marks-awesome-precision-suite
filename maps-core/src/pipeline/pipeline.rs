use opencv as cv;
use cv::prelude::*;

use super::PipelineStage;

pub type Pipeline = Vec<Box<dyn PipelineStage>>;

impl PipelineStage for Pipeline {
    fn compute(&self, image: &mut Mat) {
        for stage in self {
            stage.compute(image);
        }
    }
}