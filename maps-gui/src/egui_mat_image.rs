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
}

impl MatImage {
    pub fn new() -> Self {
        Self {
            mat: None,
            texture_id: None,
        }
    }

    // pub fn new_from_mat(mat: Mat) -> Self {
    //     let mut out = Self::new();

    //     out.set_mat(mat);

    //     out
    // }

    pub fn set_mat(&mut self, mat: Mat, ctx: &egui::Context) -> Result<(), Box<dyn Error>> {
        // Make sure the new mat is not the same as the current mat
        if let Some(current_mat) = &self.mat {
            if mat.data_bytes()? == current_mat.data_bytes()? {
                return Ok(());
            }
        }

        let color_image = match mat.channels() {
            1 => ColorImage::from_gray(
                // The from_gray and from_rgb methods copy all the image bytres in the mat
                [
                    mat.size().unwrap().width as usize,
                    mat.size().unwrap().height as usize,
                ],
                mat.data_bytes().unwrap(),
            ),
            3 => ColorImage::from_rgb(
                [
                    mat.size().unwrap().width as usize,
                    mat.size().unwrap().height as usize,
                ],
                mat.data_bytes().unwrap(),
            ),
            _ => panic!(
                "MatImage loader does not support images with {} channels",
                mat.channels()
            ),
        };

        let image_data = ImageData::Color(Arc::new(color_image));

        let texture_manager_handle = ctx.tex_manager();
        let mut texture_manager = texture_manager_handle.write();

        // Free the old texture
        if let Some(texture_id) = self.texture_id {
            texture_manager.free(texture_id);
        }

        // Allocate the new texture
        self.texture_id =
            Some(texture_manager.alloc("name".into(), image_data, TextureOptions::LINEAR));

        self.mat = Some(mat);

        Ok(())
    }

    pub fn get_texture(&self) -> Option<SizedTexture> {
        if let (Some(mat), Some(texture_id)) = (&self.mat, self.texture_id) {
            Some(SizedTexture {
                id: texture_id,
                size: Vec2::from([
                    mat.size().unwrap().width as f32 / 10.0,
                    mat.size().unwrap().height as f32 / 10.0,
                ]),
            })
        } else {
            None
        }
    }
}
