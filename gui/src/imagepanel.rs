use eframe::egui;
use egui::Ui;

pub enum PipelineStageToShow {
    ORIGINAL,
    THRESHOLDED
}

pub struct ImageViewerPanel {
    
}

impl ImageViewerPanel {
    pub fn new() -> Self {
        Self {
            
        }
    }

    pub fn draw_ui(&mut self, ui: &mut Ui) {

        ui.label("Wow so cool (an official message from the image viewer panel)");

        ui.image(egui::include_image!(
            "../../images/testtarget15.jpg"
        ));

        // This allows smooth and continuous adjustment of the sidebar size
        ui.allocate_space(ui.available_size());
    }
}

