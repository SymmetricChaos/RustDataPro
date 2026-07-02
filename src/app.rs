use crate::{
    data::{SessionData, ksf::Ksf},
    pages::Page,
    randomness_page::RandomServices,
    session_page::SessionPage,
    timers::Timers,
    utils::date_time_string,
};
use chrono::Local;
use egui::{RichText, warn_if_debug_build, widgets};
use egui_file_dialog::FileDialog;

pub struct DataPro {
    active_page: Page,
    timers_active: bool,

    ksf_file_dialog: FileDialog,
    ksf: Option<Ksf>,
    ksf_err_string: Option<String>,

    session_data_file_dialog: FileDialog,
    session_data: Option<SessionData>,
    session_data_err_string: Option<String>,

    randomness_page: RandomServices,
    data_tracking_page: SessionPage,
    timer_page: Timers,
}

impl Default for DataPro {
    fn default() -> Self {
        Self {
            active_page: Page::About,
            timers_active: false,

            ksf_file_dialog: FileDialog::new(),
            ksf: None,
            ksf_err_string: None,
            session_data_file_dialog: FileDialog::new(),
            session_data: None,
            session_data_err_string: None,

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
        self.active_page = page;
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
                        self.data_tracking_page.load_ksf(ksf);
                    } else {
                        self.data_tracking_page.load_ksf(&Ksf::default());
                    }
                    if let Some(session_data) = &self.session_data {
                        self.data_tracking_page
                            .load_session_data(session_data.clone());
                    } else {
                        self.data_tracking_page
                            .load_session_data(SessionData::default());
                    }
                }
            });
        });

        if self.timers_active {
            self.timer_page.view(ui)
        }

        match self.active_page {
            Page::About => {}
            Page::Randomness => self.randomness_page.view(ui),
            Page::DataTracking => self.data_tracking_page.view(ui, &mut self.active_page),
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
                self.ksf_file_dialog.pick_file();
            }
            if let Some(ksf) = &self.ksf {
                ui.label(format!("KSF file: {}", ksf.name));
            }
            if let Some(e) = &self.ksf_err_string {
                ui.strong(e);
            }
            self.ksf_file_dialog.update(ui.ctx());
            if let Some(path) = self.ksf_file_dialog.take_picked() {
                if path.extension().unwrap().to_str().unwrap() != "txt" {
                    self.ksf_err_string = Some(String::from("KSF files must have extension .txt"));
                } else {
                    match Ksf::from_file(path) {
                        Ok(ksf) => {
                            self.ksf = Some(ksf);
                            self.ksf_err_string = None
                        }
                        Err(e) => self.ksf_err_string = Some(e.to_string()),
                    };
                }
            }
            ui.add_space(5.0);

            if ui.button("Select Client File").clicked() {
                self.session_data_file_dialog.pick_file();
            }
            if let Some(sd) = &mut self.session_data {
                ui.text_edit_singleline(&mut sd.first_name);
                ui.text_edit_singleline(&mut sd.last_name);
                ui.text_edit_singleline(&mut sd.client_id);
            }

            if let Some(e) = &self.session_data_err_string {
                ui.strong(e);
            }
            self.session_data_file_dialog.update(ui.ctx());
            if let Some(path) = self.session_data_file_dialog.take_picked() {
                if path.extension().unwrap().to_str().unwrap() != "json" {
                    self.session_data_err_string =
                        Some(String::from("Client files must have extension .json"));
                } else {
                    match SessionData::from_file(path) {
                        Ok(sess_data) => {
                            self.session_data = Some(sess_data);
                            self.session_data_err_string = None
                        }
                        Err(e) => self.session_data_err_string = Some(e.to_string()),
                    };
                }
            }
        });
    }
}
