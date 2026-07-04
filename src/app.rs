use crate::{
    data::{ClientData, DataType, SessionData, ksf::Ksf},
    pages::Page,
    random_services::RandomServices,
    session_page::SessionPage,
    timers::Timers,
    utils::date_time_string,
};
use chrono::Local;
use egui::{RichText, warn_if_debug_build, widgets};
use egui_file_dialog::FileDialog;
use std::path::PathBuf;

pub struct Data {
    pub client: ClientData,
    pub session: SessionData,
    pub ksf: Ksf,
}

pub struct DataPro {
    data: Data,

    active_page: Page,
    timers_open: bool,
    random_open: bool,

    ksf_file_dialog: FileDialog,
    ksf_err: String,

    client_data_file_dialog: FileDialog,
    client_data_err: String,
    client_data_path: Option<String>,

    randomness_page: RandomServices,
    timers: Timers,
    session_page: SessionPage,
}

impl Default for DataPro {
    fn default() -> Self {
        Self {
            data: Data {
                client: ClientData::default(),
                session: SessionData::default(),
                ksf: Ksf::default(),
            },

            active_page: Page::About,
            timers_open: false,
            random_open: false,

            ksf_file_dialog: FileDialog::new(),
            // ksf: Ksf::default(),
            ksf_err: String::new(),

            client_data_file_dialog: FileDialog::new(),
            // client_data: ClientData::default(),
            client_data_err: String::new(),
            client_data_path: None,

            // session_data: SessionData::default(),
            randomness_page: RandomServices::default(),
            session_page: SessionPage::new(),
            timers: Timers::default(),
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

    fn load_ksf_file(&mut self, path: PathBuf) {
        match Ksf::from_file(path) {
            Ok(ksf) => {
                self.data.ksf = ksf;
                self.session_page.load_ksf(&self.data.ksf);
                self.ksf_err.clear();
            }
            Err(e) => self.ksf_err = e.to_string(),
        };
    }

    fn load_client_file(&mut self, path: PathBuf) {
        match ClientData::from_file(&path) {
            Ok(sess_data) => {
                self.client_data_path = Some(path.as_path().to_str().unwrap().to_string());
                self.data.client = sess_data;
                self.data.client.session_number += 1;
                self.client_data_err.clear();
                if self.data.client.assessments.is_empty() {
                    self.client_data_err.push_str("NO ASSESSMENTS");
                } else {
                    self.data.session.assessment = self.data.client.assessments[0].clone();
                }
                if self.data.client.conditions.is_empty() {
                    if !self.client_data_err.is_empty() {
                        self.client_data_err.push('\n');
                    }
                    self.client_data_err.push_str("NO CONDITIONS");
                } else {
                    self.data.session.condition = self.data.client.conditions[0].clone();
                }
            }
            Err(e) => self.client_data_err = e.to_string(),
        };
    }
}

impl eframe::App for DataPro {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top_panel").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.request_repaint_after_secs(5.0);
                widgets::global_theme_preference_switch(ui);
                ui.separator();

                if ui.button("About").clicked() {
                    self.set_page(Page::About);
                }

                if ui.button("Data Tracking").clicked() {
                    self.session_page.load_ksf(&self.data.ksf);
                    self.set_page(Page::DataTracking);
                }

                ui.label(format!("{}", date_time_string(Local::now())));
            });
        });

        self.timers.view(ui, &mut self.timers_open);
        self.randomness_page.view(ui, &mut self.random_open);

        match self.active_page {
            Page::About => {}
            Page::DataTracking => self.session_page.view(
                ui,
                &mut self.active_page,
                &mut self.data,
                &self.client_data_path,
            ),
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

                ui.add_space(10.0);
                if ui.button("Randomness").clicked() {
                    self.random_open = !self.random_open;
                }

                ui.add_space(10.0);
                if ui.button("Timers").clicked() {
                    self.timers_open = !self.timers_open;
                }
            });
        egui::CentralPanel::default().show(ui, |ui| {
            ui.add_space(15.0);

            // ### KSF FILE ###
            if ui.button("Select KSF File").clicked() {
                self.ksf_file_dialog.pick_file();
            }
            ui.label(format!("KSF file: {}", self.data.ksf.name));
            ui.strong(&self.ksf_err);
            self.ksf_file_dialog.update(ui.ctx());
            if let Some(path) = self.ksf_file_dialog.take_picked() {
                self.load_ksf_file(path);
            }
            ui.add_space(15.0);

            // ### CLIENT FILE ###
            if ui.button("Select Client File").clicked() {
                self.client_data_file_dialog.pick_file();
            }
            ui.strong(&self.client_data_err);
            ui.add_space(5.0);
            ui.monospace(format!("           Client: {}", &self.data.client.name));
            ui.monospace(format!(
                "               ID: {}",
                &self.data.client.client_id
            ));
            ui.monospace(format!(
                "     Case Manager: {}",
                &self.data.client.case_manager
            ));
            ui.monospace(format!(
                "Primary Therapist: {}",
                &self.data.client.primary_therapist
            ));
            ui.horizontal(|ui| {
                ui.monospace("          Session:");
                ui.add(egui::DragValue::new(&mut self.data.client.session_number))
            });
            ui.horizontal(|ui| {
                ui.monospace("        Therapist:");
                ui.text_edit_singleline(&mut self.data.session.therapist);
            });
            ui.horizontal(|ui| {
                ui.monospace("   Data Collector:");
                ui.text_edit_singleline(&mut self.data.session.data_collector);
            });

            self.client_data_file_dialog.update(ui.ctx());
            if let Some(path) = self.client_data_file_dialog.take_picked() {
                self.load_client_file(path);
            }

            // ### DROPDOWMS ###
            ui.group(|ui| {
                ui.label("Data Type");
                egui::ComboBox::from_id_salt("datatype")
                    .selected_text(self.data.session.data_type.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.data.session.data_type,
                            DataType::Primary,
                            "Primary",
                        );
                        ui.selectable_value(
                            &mut self.data.session.data_type,
                            DataType::Reliability,
                            "Reliability",
                        );
                    });
                ui.add_space(5.0);
                ui.label("Assessment");
                egui::ComboBox::from_id_salt("assessment")
                    .selected_text(&self.data.session.assessment)
                    .show_ui(ui, |ui| {
                        for item in self.data.client.assessments.iter() {
                            ui.selectable_value(
                                &mut self.data.session.assessment,
                                item.to_string(),
                                item,
                            );
                        }
                    });
                ui.add_space(5.0);
                ui.label("Condition");
                egui::ComboBox::from_id_salt("condition")
                    .selected_text(&self.data.session.condition)
                    .show_ui(ui, |ui| {
                        for item in self.data.client.conditions.iter() {
                            ui.selectable_value(
                                &mut self.data.session.condition,
                                item.to_string(),
                                item,
                            );
                        }
                    })
            });
        });
    }
}
