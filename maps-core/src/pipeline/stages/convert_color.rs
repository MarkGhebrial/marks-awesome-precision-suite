use opencv::core::MatTrait;
use opencv::imgproc::ColorConversionCodes;
use opencv::traits::Boxed;
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

        // TODO: USE THE `modify_inplace` HELPER FUNCTION INSTEAD
        // Create a second reference to the Mat without actually copying its data.
        let i = unsafe {
            // Increment the Mat's reference counter to avoid a double free
            image.addref().expect("unable to increment reference count for Mat");

            Mat::from_raw(image.as_raw_mut())
        };

        // Blur the image
        imgproc::cvt_color_def(
            &i,
            image,
            self.conversion_code.into(),
        )
        .unwrap();
    }
}
