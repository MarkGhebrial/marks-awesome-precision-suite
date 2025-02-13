use eframe::egui;

use egui::Image;
use egui::Ui;

use crate::egui_mat_image::MatImage;

pub struct ImageViewerPanel {
    image: MatImage,
}

impl ImageViewerPanel {
    pub fn new() -> Self {
        let image = MatImage::new_from_mat(maps_core::test_function());

        Self { image }
    }

    pub fn draw_ui(&mut self, ui: &mut Ui) {
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
