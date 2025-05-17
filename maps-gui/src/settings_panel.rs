use std::sync::mpsc::Sender;

use eframe::egui;

use eframe::egui::SelectableLabel;
use eframe::egui::Slider;
use egui::Frame;
use egui::Grid;
use egui::Ui;

use maps_core::parameters::*;

use crate::SharedState;

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
}

impl crate::GUIPanel for SettingsPanel {
    fn draw_ui(&mut self, ui: &mut Ui, shared_state: &mut SharedState) {
        let prev_params = self.params.clone();

        let corner_settings_frame_response = Frame::none().show(ui, |ui| {
            ui.heading("Corner settings");

            ui.label("Corner threshold mode");
            
            ui.horizontal(|ui| {
                let manual_button = ui.add(SelectableLabel::new(match self.params.corner_thresh_mode {
                    ThresholdMode::Manual { thresh: _ } => true,
                    _ => false
                }, "Manual"));
                if manual_button.clicked() {
                    self.params.corner_thresh_mode = ThresholdMode::Manual { thresh: 0.0 };
                }

                // ui.selectable_value(&mut self.params.corner_thresh_mode, ThresholdMode::Manual { thresh: 0.0 }, "Manual");
                ui.selectable_value(&mut self.params.corner_thresh_mode, ThresholdMode::Automatic { c: 0.0 }, "Automatic");
                ui.selectable_value(&mut self.params.corner_thresh_mode, ThresholdMode::Otsu, "Otsu");
            });

            match &mut self.params.corner_thresh_mode {
                ThresholdMode::Manual { thresh } => { ui.add(Slider::new(thresh, 0.0..=255.0)); },
                _ => {}
            }

            ui.set_width(ui.available_width());
        });

        ui.separator();

        let target_settings_frame_response = Frame::none().show(ui, |ui| {
            ui.heading("Target settings").on_hover_text("Tooltip");

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

        // Choose what image to display based on which area of the settings
        // panel is currently being hovered.
        if corner_settings_frame_response.response.contains_pointer() {
            ui.label("CORNER SETTINGS ARE BEING HOVERED");
            shared_state.index_of_image_to_show = 1;
        }
        else if target_settings_frame_response.response.contains_pointer() {
            ui.label("TARGET SETTINGS ARE BEING HOVERED");
            shared_state.index_of_image_to_show = 2;
        }
        else {
            shared_state.index_of_image_to_show = 0;
        }

        // Check if the user has changed the parameters
        if prev_params != self.params {
            // If the parameters have changed, send them to the pipeline thread
            self.send.send(self.params.clone()).unwrap();
        }

        println!("Target size: {:?}", self.params.target_dimensions);
    }
}
