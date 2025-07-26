use eframe::egui::{Grid, Slider, Ui};
use maps_core::parameters::ThresholdMode;

/// Egui panel that allows the user to edit the settings for binary image thresholding.
pub struct ThresholdSettingsPanel {
    /// We keep track of the manual and adaptive threshold settings because we want to remember them after
    /// the user selects a different mode and returns back to one of these ones. Does that make sense?
    /// Probably not but I'm too lazy to revise this comment
    manual_thresh: f64,
    adaptive_block_size: i32,
    adaptive_c: f64,
}

impl ThresholdSettingsPanel {
    pub fn new() -> Self {
        Self {
            manual_thresh: 0.0,
            adaptive_block_size: 3,
            adaptive_c: 0.0,
        }
    }

    pub fn draw_ui(&mut self, ui: &mut Ui, mode: &mut ThresholdMode) {
        // Draw radio button for thresholding mode
        ui.horizontal(|ui| {
            // Add the "Otsu" button to the ui. This is simple because the Otsu
            // thresholding mode does not have any parameters
            ui.selectable_value(mode, ThresholdMode::Otsu, "Otsu")
                .on_hover_text("Automatic thresholding using Otsu's algorithm (recommended)");

            ui.selectable_value(
                mode,
                ThresholdMode::Manual {
                    thresh: self.manual_thresh,
                },
                "Manual",
            )
            .on_hover_text("Simple binary thresholding with a user-defined threshold");

            // let manual_button = ui
            //     .add(SelectableLabel::new(
            //         match self.selected_mode {
            //             ThresholdMode::Manual { thresh: _ } => true,
            //             _ => false,
            //         },
            //         "Manual",
            //     ))
            //     .on_hover_text("Simple binary thresholding with a user-defined threshold");
            // if manual_button.clicked() {
            //     self.selected_mode = ThresholdMode::Manual {
            //         thresh: self.manual_thresh,
            //     };
            // }

            // ui.selectable_value(&mut self.params.selected_mode, ThresholdMode::Manual { thresh: 0.0 }, "Manual");
            ui.selectable_value(
                mode,
                ThresholdMode::Adaptive {
                    block_size: self.adaptive_block_size,
                    c: self.adaptive_c,
                },
                "Adaptive",
            )
            .on_hover_text(
                "Adaptive thresholding for images with uneven lighting (not recommended)",
            );
        });

        match mode {
            ThresholdMode::Manual { thresh } => {
                ui.add(Slider::new(thresh, 0.0..=255.0));
                self.manual_thresh = *thresh;
            }
            ThresholdMode::Adaptive { block_size, c } => {
                Grid::new("TODO: What happens when multiple instances of this struct are created and multiple grids exist with the same name?").show(ui, |ui| {
                    ui.label("Block Size");
                    ui.add(
                        Slider::new(block_size, 3..=151)
                            .clamping(eframe::egui::SliderClamping::Never),
                    );

                    ui.end_row();

                    ui.label("c:");
                    ui.add(Slider::new(c, 0.0..=50.0).step_by(0.05));
                });

                if *block_size < 3 {
                    // Make sure block size is always above the minimum
                    *block_size = 3;
                } else if *block_size % 2 == 0 {
                    // Make sure block size is always odd
                    *block_size += 1;
                }

                self.adaptive_block_size = *block_size;
                self.adaptive_c = *c;
            }
            _ => {}
        }
    }
}
