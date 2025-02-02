use eframe::egui;
use eframe::egui::load::SizedTexture;
use eframe::egui::mutex::RwLock;
use eframe::egui::Vec2;
use egui::epaint;
use egui::ImageSource;
use egui::TextureOptions;
use egui::Ui;
use epaint::ColorImage;
use epaint::ImageData;
use epaint::TextureManager;

use cv::core::Mat;
use opencv as cv;
use opencv::core::MatTraitConst;
use opencv::core::MatTraitConstManual;
use std::borrow::Cow;
use std::sync::Arc;

pub struct ImageViewerPanel {
    uri: Cow<'static, str>,
    image: Option<Mat>,
}

impl ImageViewerPanel {
    pub fn new() -> Self {
        let uri: Cow<'static, str> = Cow::from("fdafdsa");

        Self { uri, image: None }
    }

    pub fn draw_ui(&mut self, ui: &mut Ui, texture_manager: Arc<RwLock<TextureManager>>) {
        ui.label("Wow so cool (an official message from the image viewer panel)");

        // ui.image(egui::include_image!(
        //     "../../images/testtarget15.jpg"
        // ));

        if let Some(image) = &self.image {
            let color_image = ColorImage::from_rgb(
                [
                    image.size().unwrap().width as usize,
                    image.size().unwrap().height as usize,
                ],
                image.data_bytes().unwrap(),
            );

            let image_data = ImageData::Color(Arc::new(color_image));

            let texture_id =
                texture_manager
                    .write()
                    .alloc("name".into(), image_data, TextureOptions::LINEAR);

            // Show the image

            ui.image(ImageSource::Texture(SizedTexture::new(
                texture_id,
                Vec2::from([
                    image.size().unwrap().width as f32 / 10.0,
                    image.size().unwrap().height as f32/ 10.0,
                ]),
            )));

            // ui.add(widget)
        } else {
            self.image = Some(maps_core::test_function());

            ui.label("No image loaded");
        }

        // This allows smooth and continuous adjustment of the sidebar size
        ui.allocate_space(ui.available_size());
    }
}
