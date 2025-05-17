use eframe::egui;
use egui::Ui;

use opencv as cv;
use cv::core::Mat;

use std::sync::mpsc::{Receiver, Sender};

use crate::ImageViewerPanel;
use crate::SettingsPanel;

use maps_core::parameters::MAPSPipelineParams;

/// A trait for sections of the GUI that need access to the shared application
/// state.
pub trait GUIPanel {
    fn draw_ui(&mut self, ui: &mut Ui, shared_state: &mut SharedState);
}

/// State shared between different elements of the GUI
pub struct SharedState {
    /// The file path of the image to load
    pub file_path: String,

    /// fdsaf
    pub index_of_image_to_show: usize,
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            file_path: "/home/markg/Documents/Code/Marks-Awesome-Precision-Suite/images/testtarget15.jpg".into(),
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
    pub fn new(recv: Receiver<Vec<(String, Mat)>>, send: Sender<MAPSPipelineParams>) -> Self {
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
