use std::sync::mpsc::Receiver;

use eframe::egui;

use eframe::egui::ImageSource;
use egui::Image;
use egui::Ui;

use cv::core::Mat;
use opencv as cv;

use crate::egui_mat_image::MatImage;

pub struct ImageViewerPanel {
    recv: Receiver<Vec<(String, Mat)>>,
    image: MatImage,
    dropdown_selection: String,
}

impl ImageViewerPanel {
    pub fn new(recv: Receiver<Vec<(String, Mat)>>) -> Self {
        let image = MatImage::new();

        Self {
            recv,
            image,
            dropdown_selection: "".into(),
        }
    }

    pub fn draw_ui(&mut self, ui: &mut Ui) {
        if let Ok(v) = self.recv.try_recv() {
            self.image
                .set_mat(v[2].1.clone(), &ui.ctx())
                .expect("Error updating image");
        }

        egui::ComboBox::from_label("Select one!")
            .selected_text(format!("{}", self.dropdown_selection))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.dropdown_selection, "Selection 1".into(), "First");
                ui.selectable_value(&mut self.dropdown_selection, "Selection 2".into(), "Second");
                ui.selectable_value(&mut self.dropdown_selection, "Selection 3".into(), "Third");
            });

        ui.label("Wow so cool (an official message from the image viewer panel)");

        match self.image.get_texture() {
            Some(texture) => {
                let image_widget_response =
                    ui.add(Image::new(ImageSource::Texture(texture)).shrink_to_fit());
                // let rect = image_widget_response.rect;
                // rect.x_range();

                let hover_pos = image_widget_response.hover_pos();
                match hover_pos {
                    Some(pos) => {
                        ui.label(format!("Position: x={}, y={}", pos.x, pos.y));
                    }
                    None => {
                        ui.label("Not hovered");
                    }
                }
            }
            None => {
                ui.label("No images loaded");
            }
        }

        // This allows smooth and continuous adjustment of the sidebar size
        ui.allocate_space(ui.available_size());
    }
}
