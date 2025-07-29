use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::time::Instant;

use eframe::egui;

use eframe::egui::ImageSource;
use egui::Image;
use egui::Ui;

use cv::core::Mat;
use opencv as cv;

use crate::app::GUIPanel;
use crate::app::SharedState;
use crate::egui_mat_image::MatImage;
use crate::util;

pub struct ImageViewerPanel {
    recv: Receiver<Vec<(String, Arc<Mat>)>>,
    received_images: Option<Vec<(String, Arc<Mat>)>>,

    image: MatImage,
}

impl ImageViewerPanel {
    pub fn new(recv: Receiver<Vec<(String, Arc<Mat>)>>) -> Self {
        let image = MatImage::new();

        Self {
            recv,
            received_images: None,
            image,
            // dropdown_selection: "".into(),
        }
    }
}

impl GUIPanel for ImageViewerPanel {
    fn draw_ui(&mut self, ui: &mut Ui, shared_state: &mut SharedState) {
        println!(
            "Index of image to show: {}",
            shared_state.index_of_image_to_show
        );

        // Receive images from the channel.
        // TODO: Consider moving this into the App struct
        if let Ok(v) = self.recv.try_recv() {
            self.received_images = Some(v);
        }

        let start = Instant::now();
        if let Some(images) = &self.received_images {
            // Display the relevant image to the screen
            self.image
                .set_mat(
                    Arc::clone(&images[shared_state.index_of_image_to_show].1),
                    &ui.ctx(),
                )
                .expect("Error updating image");
        }
        let elapsed = start.elapsed();
        println!("Took {} seconds to set mat", elapsed.as_secs_f64());

        match self.image.get_texture() {
            Some(texture) => {
                let image_widget_response =
                    ui.add(Image::new(ImageSource::Texture(texture)).shrink_to_fit());
                // let rect = image_widget_response.rect;
                // rect.x_range();

                let hover_pos = image_widget_response.hover_pos();
                match hover_pos {
                    Some(pos) => {
                        // Map the egui coordinates to the coordinate space of the target
                        let rect = image_widget_response.interact_rect;
                        let x_position: f32 = util::map(
                            pos.x,
                            rect.min.x..rect.max.x,
                            0.0..shared_state.params.target_dimensions.1 as f32,
                        );
                        let y_position: f32 = util::map(
                            pos.y,
                            rect.min.y..rect.max.y,
                            0.0..shared_state.params.target_dimensions.1 as f32,
                        );
                        ui.label(format!("Position: x={}, y={}", x_position, y_position));
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
