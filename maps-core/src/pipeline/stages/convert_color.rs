use opencv::core::ModifyInplace;
use opencv::imgproc::ColorConversionCodes;
use opencv as cv;
use cv::core::Mat;
use cv::imgproc;

use super::super::PipelineStage;

pub struct ConvertColorStage {
    pub conversion_code: ColorConversionCodes,
}

impl ConvertColorStage {
    pub fn rgba_to_greyscale() -> Self {
        Self {
            conversion_code: ColorConversionCodes::COLOR_BGR2GRAY,
        }
    }

    pub fn greyscale_to_rgba() -> Self {
        Self {
            conversion_code: ColorConversionCodes::COLOR_GRAY2BGR,
        }
    }
}

impl PipelineStage for ConvertColorStage {
    fn compute(&self, image: &mut Mat) {
        // We have to do this unsafe `modify_inplace` nonsense because in-place opencv
        // operations require both an immutable reference and a mutable reference
        // to the same Mat. Safe Rust does not allow that.
        //
        // See https://github.com/twistedfall/opencv-rust/issues/571
        //
        // TODO: Figure out what color conversions are not safe.
        unsafe {
            image.modify_inplace(|input, output| {
                imgproc::cvt_color_def(
                    input,
                    output,
                    self.conversion_code.into(),
                )
                .unwrap();
            })
        }
    }
}
