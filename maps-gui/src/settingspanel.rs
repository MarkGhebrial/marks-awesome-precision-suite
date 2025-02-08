use eframe::egui;

use eframe::egui::Slider;
use egui::Frame;
use egui::Grid;
use egui::Ui;

use maps_core::pipeline::MAPSPipelineParams;

pub struct SettingsPanel {
    params: MAPSPipelineParams,
}

impl SettingsPanel {
    pub fn new() -> Self {
        Self {
            params: MAPSPipelineParams::default(),
        }
    }

    pub fn draw_ui(&mut self, ui: &mut Ui) {


        let inner_response = Frame::none().show(ui, |ui| {
            Grid::new("settings panel grid").show(ui, |ui| {
                ui.label("Target height: ");
    
                ui.add(Slider::new(
                    &mut self.params.target_dimensions.0,
                    0.0..=30.0,
                ));
                ui.end_row();
    
                ui.label("Target width: ");
    
                ui.add(Slider::new(
                    &mut self.params.target_dimensions.1,
                    0.0..=30.0,
                ));
                ui.end_row();
            });
        });

        if inner_response.response.hovered() {
            ui.label("FRAME IS BEING HOVERED");
        }



        println!("Target size: {:?}", self.params.target_dimensions);
    }
}
