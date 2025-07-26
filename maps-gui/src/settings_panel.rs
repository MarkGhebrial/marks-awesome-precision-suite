use std::sync::mpsc::Sender;

use eframe::egui;

use eframe::egui::Slider;
use egui::Context;
use egui::Frame;
use egui::Grid;
use egui::Ui;

use maps_core::parameters::*;

// use crate::adaptive_threshold_settings_panel::AdaptiveThresholdSettingsPanel;
use crate::app::GUIPanel;
use crate::app::SharedState;
use crate::threshold_settings_panel::ThresholdSettingsPanel;

pub struct SettingsPanel {
    send: Sender<(Context, MAPSPipelineParams)>,
    corner_threshold_panel: ThresholdSettingsPanel,
}

impl SettingsPanel {
    pub fn new(send: Sender<(Context, MAPSPipelineParams)>) -> Self {
        Self {
            send,
            corner_threshold_panel: ThresholdSettingsPanel::new(),
        }
    }
}

impl GUIPanel for SettingsPanel {
    fn draw_ui(&mut self, ui: &mut Ui, shared_state: &mut SharedState) {
        let prev_params = shared_state.params.clone();

        let corner_settings_frame_response = Frame::none().show(ui, |ui| {
            ui.heading("Corner Detection Settings");

            ui.label("Corner threshold mode:");

            self.corner_threshold_panel
                .draw_ui(ui, &mut shared_state.params.corner_thresh_mode);

            // Make the frame fill the full width of the panel
            ui.set_width(ui.available_width());
        });

        ui.separator();

        let target_settings_frame_response = Frame::none().show(ui, |ui| {
            ui.heading("Target Settings").on_hover_text("Tooltip");

            Grid::new("settings panel grid").show(ui, |ui| {
                ui.label("Target height: ");

                ui.add(Slider::new(
                    &mut shared_state.params.target_dimensions.1,
                    0.0..=30.0,
                ));
                ui.end_row();

                ui.label("Target width: ");

                ui.add(Slider::new(
                    &mut shared_state.params.target_dimensions.0,
                    0.0..=30.0,
                ));
                ui.end_row();
            });

            if ui.button("Swap width and height").clicked() {
                let temp = shared_state.params.target_dimensions.0;
                shared_state.params.target_dimensions.0 = shared_state.params.target_dimensions.1;
                shared_state.params.target_dimensions.1 = temp;
            }
        });

        ui.separator();

        let dot_detection_settings_frame_response = Frame::none().show(ui, |ui| {
            ui.heading("Dot Detection Settings");

            // self.corner_threshold_panel.draw_ui(ui, shared_state);
        });

        // Choose what image to display based on which area of the settings
        // panel is currently being hovered.
        if corner_settings_frame_response.response.contains_pointer() {
            ui.label("CORNER SETTINGS ARE BEING HOVERED");
            shared_state.index_of_image_to_show = 1;
        } else if target_settings_frame_response.response.contains_pointer() {
            ui.label("TARGET SETTINGS ARE BEING HOVERED");
            shared_state.index_of_image_to_show = 2;
        } else if dot_detection_settings_frame_response
            .response
            .contains_pointer()
        {
            ui.label("DOT DETECTION SETTINGS ARE BEING HOVERED");
            shared_state.index_of_image_to_show = 3;
        } else {
            shared_state.index_of_image_to_show = 0;
        }

        // Check if the user has changed the parameters
        if prev_params != shared_state.params {
            // If the parameters have changed, send them to the pipeline thread
            if self
                .send
                .send((ui.ctx().clone(), shared_state.params.clone()))
                .is_err()
            {
                println!("Failed to send params to image processing thread");
                // TODO: This should probably panic
            }
        }

        println!("Target size: {:?}", shared_state.params.target_dimensions);
    }
}
