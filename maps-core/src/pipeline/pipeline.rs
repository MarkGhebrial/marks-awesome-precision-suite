use opencv as cv;
use cv::prelude::*;

use super::PipelineStage;

type Pipeline = Vec<Box<dyn PipelineStage>>;

impl PipelineStage for Pipeline {
    fn compute(&self, image: Mat) -> Mat {
        let mut out = image.clone();

        for stage in self {
            out = stage.compute(out);
        }

        out
    }
}