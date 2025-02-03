use std::error::Error;
use std::sync::Arc;

use cv::core::Mat;
use cv::core::MatTraitConst;
use cv::core::MatTraitConstManual;
use opencv as cv;

use eframe::egui::{self, load::SizedTexture, ImageSource, TextureId, Vec2};
use egui::epaint::TextureManager;
use egui::mutex::RwLock;
use egui::ColorImage;
use egui::ImageData;
use egui::TextureOptions;

// This module converts images from opencv Mats to egui-compatible types.

pub struct MatImage {
    mat: Option<Mat>,
    texture_id: Option<TextureId>,
    texture_id_up_to_date: bool,
}

impl MatImage {
    pub fn new() -> Self {
        Self {
            mat: None,
            texture_id: None,
            texture_id_up_to_date: false,
        }
    }

    pub fn new_from_mat(mat: Mat) -> Self {
        Self {
            mat: Some(mat),
            texture_id: None,
            texture_id_up_to_date: false,
        }
    }

    pub fn set_mat(&mut self, new_mat: Mat) -> Result<(), Box<dyn Error>> {
        if let Some(current_mat) = &self.mat {
            if new_mat.data_bytes()? == current_mat.data_bytes()? {
                // The new mat is the same as the old mat. No need to take action
                return Ok(());
            } else {
                self.mat = Some(new_mat);
                self.texture_id_up_to_date = false
            }
        }

        Ok(())
    }

    pub fn get_image_source(
        &mut self,
        texture_manager: Arc<RwLock<TextureManager>>,
    ) -> Option<ImageSource> {
        if let Some(mat) = &self.mat {
            if !self.texture_id_up_to_date || self.texture_id.is_none() {
                let color_image = ColorImage::from_rgb(
                    [
                        mat.size().unwrap().width as usize,
                        mat.size().unwrap().height as usize,
                    ],
                    mat.data_bytes().unwrap(),
                );

                let image_data = ImageData::Color(Arc::new(color_image));

                let mut texture_manager = texture_manager.write();

                if let Some(texture_id) = self.texture_id {
                    texture_manager.free(texture_id);
                }

                self.texture_id =
                    Some(texture_manager.alloc("name".into(), image_data, TextureOptions::LINEAR));
                self.texture_id_up_to_date = true;
            }

            return Some(ImageSource::Texture(SizedTexture::new(
                self.texture_id.unwrap(),
                Vec2::from([
                    mat.size().unwrap().width as f32 / 10.0,
                    mat.size().unwrap().height as f32 / 10.0,
                ]),
            )));
        }

        None
    }
}
