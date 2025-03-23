use std::borrow::Cow;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;

use eframe::egui;

use eframe::egui::ImageSource;
use egui::Image;
use egui::Ui;
use egui::load::Bytes;

use cv::core::Mat;
use opencv::core::MatTraitConstManual;
use opencv as cv;

use crate::egui_mat_image::MatImage;

pub struct ImageViewerPanel {
    recv: Receiver<Mat>,
    image: MatImage,
}

impl ImageViewerPanel {
    pub fn new(recv: Receiver<Mat>) -> Self {
        let image = MatImage::new();

        Self { recv, image }
    }

    pub fn draw_ui(&mut self, ui: &mut Ui) {
        if let Ok(mat) = self.recv.try_recv() {
            self.image.set_mat(mat, &ui.ctx()).expect("Error updating image");
        }

        ui.label("Wow so cool (an official message from the image viewer panel)");

        match self.image.get_texture() {
            Some(texture) => {
                ui.add(Image::new(ImageSource::Texture(texture)).shrink_to_fit());
            }
            None => {
                ui.label("No image loaded");
            }
        }

        // This allows smooth and continuous adjustment of the sidebar size
        ui.allocate_space(ui.available_size());
    }
}
