use crate::{
    about_page::About,
    data_tracking_page::DataTracking,
    ksf::{Keybind, Ksf},
    pages::Page,
    randomness_page::RandomServices,
};
use egui::widgets;

pub struct TemplateApp {
    active_page: Page,

    loaded_ksf: Ksf,

    about_page: About,
    randomness_page: RandomServices,
    data_tracking_page: DataTracking,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let temp_ksf = Ksf {
            duration: vec![
                Keybind::from_string("1,Sr+"),
                Keybind::from_string("5,Sdelta"),
            ],
            frequency: vec![
                Keybind::from_string("a,Aggression"),
                Keybind::from_string("s,SIB"),
            ],
        };
        Self {
            active_page: Page::About,
            loaded_ksf: temp_ksf,
            about_page: About::default(),
            randomness_page: RandomServices::default(),
            data_tracking_page: DataTracking::default(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(2.0);
        Default::default()
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
                    self.data_tracking_page.load_ksf(&self.loaded_ksf);
                    self.active_page = Page::DataTracking;
                }
            });
        });

        match self.active_page {
            Page::About => self.about_page.view(ui),
            Page::Randomness => self.randomness_page.view(ui),
            Page::DataTracking => self.data_tracking_page.view(ui),
        }
    }
}
