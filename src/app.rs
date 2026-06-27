use chrono::{Datelike, Local, Timelike};
use egui::{RichText, Ui, warn_if_debug_build, widgets};

use crate::{data_tracking_page::DataTrackingPage, pages::Page, randomness_page::RandomnessPage};

pub struct TemplateApp {
    active_page: Page,
    randomness_page: RandomnessPage,
    data_tracking_page: DataTrackingPage,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            active_page: Page::About,
            randomness_page: RandomnessPage::default(),
            data_tracking_page: DataTrackingPage::default(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(2.0);
        Default::default()
    }

    fn about_page(&mut self, ui: &mut Ui) {
        egui::Panel::left("about_display_panel")
            .default_size(500.0)
            .min_size(200.0)
            .show_inside(ui, |ui| {
                warn_if_debug_build(ui);
                let hello = RichText::new("Welcome to RustDataPro!").strong();
                ui.label(hello);
                ui.add_space(20.0);
                // ui.hyperlink_to(
                //     "source code",
                //     "https://github.com/SymmetricChaos/crypto-gui",
                // );
                // ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("Powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.label(
                RichText::new("Data Collection and Management")
                    .heading()
                    .strong(),
            );
            let dt = Local::now();

            ui.label(format!(
                "The Current Date/Time is {} {}/{}/{} {:02}:{:02}",
                dt.weekday(),
                dt.month(),
                dt.day(),
                dt.year(),
                dt.hour(),
                dt.minute(),
            ));
        });
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                widgets::global_theme_preference_switch(ui);
                ui.separator();

                if ui.button("About").clicked() {
                    self.active_page = Page::About;
                }

                if ui.button("Randomness").clicked() {
                    self.active_page = Page::Randomness;
                }

                if ui.button("Data Tracking").clicked() {
                    self.active_page = Page::DataTracking;
                }
            });
        });

        match self.active_page {
            Page::About => self.about_page(ui),
            Page::Randomness => self.randomness_page.view(ui),
            Page::DataTracking => self.data_tracking_page.view(ui),
        }
    }
}
