use cv::prelude::*;
use opencv as cv;

use super::PipelineStage;

pub struct Pipeline {
    stages: Vec<Box<dyn PipelineStage>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self { stages: vec![] }
    }

    pub fn from_vec(stages: Vec<Box<dyn PipelineStage>>) -> Self {
        Self { stages }
    }

    pub fn add_stage<U: PipelineStage + 'static>(&mut self, stage: U) {
        self.stages.push(Box::new(stage));
    }
}

impl PipelineStage for Pipeline {
    fn compute(&self, image: &mut Mat) {
        for stage in &self.stages {
            stage.compute(image);
        }
    }
}
