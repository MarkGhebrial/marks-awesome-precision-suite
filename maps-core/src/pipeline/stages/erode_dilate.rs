use cv::core::Mat;
use cv::core::ModifyInplace;
use cv::core::Size;
use cv::core::ToInputArray;
use cv::imgproc;
use opencv as cv;

use crate::pipeline::PipelineStage;

pub enum ErodeOrDilateOperation {
    Erode,
    Dilate,
    ErodeThenDilate,
    DilateThenErode,
}

/// Perform an erosion or dilation on an image.
///
/// TODO: Copy opencv docs to the pipeline stage structs
pub struct ErodeOrDilateStage<K: ToInputArray> {
    operation: ErodeOrDilateOperation,
    kernel: K,
}

// impl Default for ErodeOrDilateStage<Mat> {
//     fn default() -> Self {
//         Self {
//             kernel: imgproc::get_structuring_element_def(imgproc::MORPH_ELLIPSE, Size::new(30, 30)).unwrap()
//         }
//     }
// }

impl ErodeOrDilateStage<Mat> {
    /// Create a new ErodeStage using a circular kernel.
    pub fn circular_kernel(operation: ErodeOrDilateOperation, diameter: i32) -> Self {
        Self {
            operation,
            kernel: imgproc::get_structuring_element_def(
                imgproc::MORPH_ELLIPSE,
                Size::new(diameter, diameter),
            )
            .unwrap(),
        }
    }
}

impl<K: ToInputArray> ErodeOrDilateStage<K> {
    /// Create a new ErodeOrDilateStage with an arbitrary kernel. Typically, `[opencv::imgproc::get_structuring_element]`
    /// is used to generate the kernel.
    fn new(operation: ErodeOrDilateOperation, kernel: K) -> Self {
        Self { operation, kernel }
    }

    fn set_kernel(mut self, kernel: K) -> Self {
        self.kernel = kernel;
        self
    }
}

impl<K: ToInputArray> PipelineStage for ErodeOrDilateStage<K> {
    fn compute(&self, image: &mut Mat) {
        let erode = |image: &mut Mat| unsafe {
            image
                .modify_inplace(|input, output| imgproc::erode_def(input, output, &self.kernel))
                .unwrap();
        };

        let dilate = |image: &mut Mat| unsafe {
            image
                .modify_inplace(|input, output| imgproc::dilate_def(input, output, &self.kernel))
                .unwrap();
        };

        use ErodeOrDilateOperation::*;
        match self.operation {
            Erode => erode(image),
            Dilate => dilate(image),
            ErodeThenDilate => {
                erode(image);
                dilate(image);
            }
            DilateThenErode => {
                dilate(image);
                erode(image);
            }
        }
    }
}
