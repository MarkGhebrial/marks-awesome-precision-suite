use std::sync::mpsc::{self, Receiver, Sender};

use eframe::egui;

use eframe::egui::Slider;
use egui::Frame;
use egui::Grid;
use egui::Ui;

use maps_core::parameters::MAPSPipelineParams;

pub struct SettingsPanel {
    send: Sender<MAPSPipelineParams>,
    params: MAPSPipelineParams,
}

impl SettingsPanel {
    pub fn new(send: Sender<MAPSPipelineParams>) -> Self {
        Self {
            send,
            params: MAPSPipelineParams::default(),
        }
    }

    pub fn draw_ui(&mut self, ui: &mut Ui) {
        let prev_params = self.params;

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

        if inner_response.response.contains_pointer() {
            ui.label("FRAME IS BEING HOVERED");
        }

        // Check if the user has changed the parameters
        if prev_params != self.params {
            // If the parameters have changed, send them to the pipeline thread
            self.send.send(self.params.clone()).unwrap();
        }

        println!("Target size: {:?}", self.params.target_dimensions);
    }
}
