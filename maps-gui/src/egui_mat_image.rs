use std::error::Error;
use std::sync::Arc;

use cv::core::Mat;
use opencv as cv;

use eframe::egui::{self, load::SizedTexture, TextureId, Vec2};
use egui::ImageData;
use egui::TextureOptions;

///! This module converts images from opencv Mats to egui-compatible types.

pub struct MatImage {
    mat: Option<Arc<Mat>>,
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

    pub fn set_mat(&mut self, mat: Arc<Mat>, ctx: &egui::Context) -> Result<(), Box<dyn Error>> {
        // let start = Instant::now();
        let texture_manager_handle = ctx.tex_manager();
        let mut texture_manager = texture_manager_handle.write();
        // let elapsed = start.elapsed();
        // println!("Took {} seconds to acquire lock for texture manager", elapsed.as_secs_f64());

        // Free the old texture
        // let start = Instant::now();
        if let Some(texture_id) = self.texture_id {
            texture_manager.free(texture_id);
        }
        // let elapsed = start.elapsed();
        // println!("Took {} seconds to free the old texture", elapsed.as_secs_f64());

        // Allocate the new texture
        // let start = Instant::now();

        self.texture_id =
            Some(texture_manager.alloc("name".into(), Arc::clone(&mat), TextureOptions::LINEAR));
        // let elapsed = start.elapsed();
        // println!("Took {} seconds to clone the mat and allocate the texture", elapsed.as_secs_f64());

        self.mat = Some(mat);
        Ok(())
    }

    pub fn get_texture(&self) -> Option<SizedTexture> {
        if let (Some(mat), Some(texture_id)) = (&self.mat, self.texture_id) {
            Some(SizedTexture {
                id: texture_id,
                size: Vec2::from([
                    ImageData::size(mat)[0] as f32,
                    ImageData::size(mat)[1] as f32,
                ]),
            })
        } else {
            None
        }
    }
}
