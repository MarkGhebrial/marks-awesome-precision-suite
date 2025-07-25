use std::sync::mpsc::Sender;

use eframe::egui;

use eframe::egui::SelectableLabel;
use eframe::egui::Slider;
use egui::Context;
use egui::Frame;
use egui::Grid;
use egui::Ui;

use maps_core::parameters::*;

use crate::app::GUIPanel;
use crate::app::SharedState;

pub struct SettingsPanel {
    send: Sender<(Context, MAPSPipelineParams)>,
    /// Saves the value of the corner manual thresholding slider
    prev_manual_thresh: f64,
}

impl SettingsPanel {
    pub fn new(send: Sender<(Context, MAPSPipelineParams)>) -> Self {
        Self {
            send,
            prev_manual_thresh: 160.0,
        }
    }
}

impl GUIPanel for SettingsPanel {
    fn draw_ui(&mut self, ui: &mut Ui, shared_state: &mut SharedState) {
        let prev_params = shared_state.params.clone();

        let corner_settings_frame_response = Frame::none().show(ui, |ui| {
            ui.heading("Corner settings");

            ui.label("Corner threshold mode:");

            // Draw radio button for thresholding mode
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut shared_state.params.corner_thresh_mode,
                    ThresholdMode::Otsu,
                    "Otsu",
                )
                .on_hover_text("Automatic thresholding using Otsu's algorithm (recommended)");

                let manual_button = ui
                    .add(SelectableLabel::new(
                        match shared_state.params.corner_thresh_mode {
                            ThresholdMode::Manual { thresh: _ } => true,
                            _ => false,
                        },
                        "Manual",
                    ))
                    .on_hover_text("Simple binary thresholding with a user-defined threshold");
                if manual_button.clicked() {
                    shared_state.params.corner_thresh_mode = ThresholdMode::Manual {
                        thresh: self.prev_manual_thresh,
                    };
                }

                // ui.selectable_value(&mut self.params.corner_thresh_mode, ThresholdMode::Manual { thresh: 0.0 }, "Manual");
                ui.selectable_value(
                    &mut shared_state.params.corner_thresh_mode,
                    ThresholdMode::Adaptive { thresh: 0.0 },
                    "Adaptive",
                )
                .on_hover_text(
                    "Adaptive thresholding for images with uneven lighting (not recommended)",
                );
            });

            match &mut shared_state.params.corner_thresh_mode {
                ThresholdMode::Manual { thresh } => {
                    ui.add(Slider::new(thresh, 0.0..=255.0));
                    self.prev_manual_thresh = *thresh;
                }
                _ => {}
            }

            // Make the frame fill the full width of the panel
            ui.set_width(ui.available_width());
        });

        ui.separator();

        let target_settings_frame_response = Frame::none().show(ui, |ui| {
            ui.heading("Target settings").on_hover_text("Tooltip");

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

        // Choose what image to display based on which area of the settings
        // panel is currently being hovered.
        if corner_settings_frame_response.response.contains_pointer() {
            ui.label("CORNER SETTINGS ARE BEING HOVERED");
            shared_state.index_of_image_to_show = 1;
        } else if target_settings_frame_response.response.contains_pointer() {
            ui.label("TARGET SETTINGS ARE BEING HOVERED");
            shared_state.index_of_image_to_show = 2;
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
            }
        }

        println!("Target size: {:?}", shared_state.params.target_dimensions);
    }
}
