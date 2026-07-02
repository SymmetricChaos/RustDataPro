use std::{cell::RefCell, rc::Rc};

use crate::{
    data::ksf::Ksf, pages::Page, randomness_page::RandomServices, session_page::SessionPage,
    timers::Timers, utils::date_time_string,
};
use chrono::Local;
use egui::{RichText, warn_if_debug_build, widgets};
use egui_file_dialog::FileDialog;

pub struct DataPro {
    active_page: Rc<RefCell<Page>>,
    timers_active: bool,

    file_dialog: FileDialog,
    ksf: Option<Ksf>,
    file_err_string: Option<String>,

    randomness_page: RandomServices,
    data_tracking_page: SessionPage,
    timer_page: Timers,
}

impl Default for DataPro {
    fn default() -> Self {
        let active_page = Rc::new(RefCell::new(Page::About));
        Self {
            active_page: active_page.clone(),
            timers_active: false,

            file_dialog: FileDialog::new(),
            ksf: None,
            file_err_string: None,

            randomness_page: RandomServices::default(),
            data_tracking_page: SessionPage::new(),
            timer_page: Timers::default(),
        }
    }
}

impl DataPro {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.5);
        Default::default()
    }

    fn set_page(&mut self, page: Page) {
        *self.active_page.borrow_mut() = page;
    }
}

impl eframe::App for DataPro {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top_panel").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                widgets::global_theme_preference_switch(ui);
                ui.separator();

                if ui.button("About").clicked() {
                    self.set_page(Page::About);
                }

                if ui.button("Randomness").clicked() {
                    self.set_page(Page::Randomness);
                }

                if ui.button("Timers").clicked() {
                    self.timers_active = !self.timers_active;
                }

                if ui.button("Data Tracking").clicked() {
                    self.set_page(Page::DataTracking);
                    if let Some(ksf) = &self.ksf {
                        self.data_tracking_page.load(ksf);
                    } else {
                        self.data_tracking_page.load(&Ksf::default());
                    }
                }
            });
        });

        if self.timers_active {
            self.timer_page.view(ui)
        }

        match self.active_page.borrow().clone() {
            Page::About => {}
            Page::Randomness => self.randomness_page.view(ui),
            Page::DataTracking => self.data_tracking_page.view(ui),
        }

        egui::Panel::left("welcome_panel")
            .default_size(500.0)
            .min_size(200.0)
            .show(ui, |ui| {
                warn_if_debug_build(ui);
                let hello =
                    RichText::new("Welcome to RustDataPro! Its kind of like BDataPro!").strong();
                ui.label(hello);
                ui.add_space(20.0);
                ui.hyperlink_to(
                    "source code",
                    "https://github.com/SymmetricChaos/RustDataPro",
                );
                ui.add_space(10.0);
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
        egui::CentralPanel::default().show(ui, |ui| {
            ui.request_repaint_after_secs(5.0);
            ui.label(format!(
                "The Current Date/Time is {}",
                date_time_string(Local::now()),
            ));

            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);

            if ui.button("Select KSF File").clicked() {
                self.file_dialog.pick_file();
            }

            if let Some(ksf) = &self.ksf {
                ui.label(format!("KSF file: {}", ksf.name));
            }
            if let Some(e) = &self.file_err_string {
                ui.strong(e);
            }

            self.file_dialog.update(ui.ctx());
            if let Some(path) = self.file_dialog.take_picked() {
                if path.extension().unwrap().to_str().unwrap() != "txt" {
                    self.file_err_string = Some(String::from("KSF files must have extension .txt"));
                } else {
                    match Ksf::from_file(path) {
                        Ok(ksf) => {
                            self.ksf = Some(ksf);
                            self.file_err_string = None
                        }
                        Err(e) => self.file_err_string = Some(e.to_string()),
                    };
                }
            }
        });
    }
}
