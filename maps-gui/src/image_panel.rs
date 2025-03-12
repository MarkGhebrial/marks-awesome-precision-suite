use std::sync::mpsc::{self, Receiver, Sender};

use eframe::egui;

use egui::Image;
use egui::Ui;

use cv::core::Mat;
use opencv as cv;

use crate::egui_mat_image::MatImage;

pub struct ImageViewerPanel {
    recv: Receiver<Mat>,
    image: MatImage,
}

impl ImageViewerPanel {
    pub fn new(recv: Receiver<Mat>) -> Self {
        let image = MatImage::new_from_mat(maps_core::test_function());

        Self { recv, image }
    }

    pub fn draw_ui(&mut self, ui: &mut Ui) {
        if let Ok(mat) = self.recv.try_recv() {
            self.image.set_mat(mat).expect("Error updating image");
        }

        ui.label("Wow so cool (an official message from the image viewer panel)");

        match self.image.get_image_source(ui.ctx().tex_manager()) {
            Some(image_source) => {
                ui.add(Image::new(image_source).shrink_to_fit());
            }
            None => {
                ui.label("No image loaded");
            }
        }

        // This allows smooth and continuous adjustment of the sidebar size
        ui.allocate_space(ui.available_size());
    }
}
