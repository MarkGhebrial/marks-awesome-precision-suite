use eframe::egui;
use egui::Context;
use egui::Ui;

use cv::core::Mat;
use opencv as cv;

use std::sync::mpsc::{Receiver, Sender};

use crate::ImageViewerPanel;
use crate::SettingsPanel;

use maps_core::parameters::MAPSPipelineParams;

/// A trait for sections of the GUI that need access to the shared application
/// state.
pub trait GUIPanel {
    fn draw_ui(&mut self, ui: &mut Ui, shared_state: &mut SharedState);
}

/// State shared between different panels of the GUI.
///
/// An instance of this struct is stored in the App struct, and is "injected"
/// into the different parts of the GUI via the [`GUIPanel`] trait.
///
/// State that's only used by one panel should not be stored in this struct.
pub struct SharedState {
    /// The file path of the image to load
    pub file_path: String,

    /// TODO: Think of a better solution
    pub index_of_image_to_show: usize,
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            file_path:
                "/home/markg/Documents/Code/Marks-Awesome-Precision-Suite/images/testtarget15.jpg"
                    .into(),
            index_of_image_to_show: 0,
        }
    }
}

pub struct MyApp {
    state: SharedState,

    image_viewer_panel: ImageViewerPanel,
    settings_panel: SettingsPanel,
}

impl MyApp {
    pub fn new(
        recv: Receiver<Vec<(String, Mat)>>,
        send: Sender<(Context, MAPSPipelineParams)>,
    ) -> Self {
        Self {
            state: SharedState::default(),
            image_viewer_panel: ImageViewerPanel::new(recv),
            settings_panel: SettingsPanel::new(send),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Draw bottom panel
        egui::TopBottomPanel::bottom("bottom panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Bottom panel");
                ui.label("This is some text. Lorem Ipsum amirite?");

                ui.horizontal(|ui| {
                    ui.label("File path for image: ");
                    ui.text_edit_singleline(&mut self.state.file_path);
                });

                // Allocate all the available space so the panel doesn't snap back
                // to its original size when the user finishes resizing it.
                ui.allocate_space(ui.available_size());
            });

        // Draw right panel
        egui::SidePanel::right("right panel")
            .resizable(false)
            .show(ctx, |ui| {
                self.settings_panel.draw_ui(ui, &mut self.state);
            });

        // Draw image viewer panel
        egui::CentralPanel::default().show(ctx, |ui| {
            // self.settings_panel.draw_ui(ui);
            self.image_viewer_panel.draw_ui(ui, &mut self.state);
        });
    }
}
