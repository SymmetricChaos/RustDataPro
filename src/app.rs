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

pub struct DataPro {
    active_page: Page,
    timers_open: bool,
    random_open: bool,

    ksf_file_dialog: FileDialog,
    ksf: Ksf,
    ksf_err: Option<String>,

    client_data_file_dialog: FileDialog,
    client_data: ClientData,
    client_data_err: Option<String>,

    session_data: SessionData,

    randomness_page: RandomServices,
    session_page: SessionPage,
    timers: Timers,
}

impl Default for DataPro {
    fn default() -> Self {
        Self {
            active_page: Page::About,
            timers_open: false,
            random_open: false,

            ksf_file_dialog: FileDialog::new(),
            ksf: Ksf::default(),
            ksf_err: None,

            client_data_file_dialog: FileDialog::new(),
            client_data: ClientData::default(),
            client_data_err: None,

            session_data: SessionData::default(),

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
                    self.random_open = !self.random_open;
                }

                if ui.button("Timers").clicked() {
                    self.timers_open = !self.timers_open;
                }

                if ui.button("Data Tracking").clicked() {
                    self.session_page.load_ksf(&self.ksf);
                    self.set_page(Page::DataTracking);
                }
            });
        });

        self.timers.view(ui, &mut self.timers_open);
        self.randomness_page.view(ui, &mut self.random_open);

        match self.active_page {
            Page::About => {}
            Page::DataTracking => self.session_page.view(
                ui,
                &mut self.active_page,
                &mut self.client_data,
                &mut self.session_data,
                &mut self.ksf,
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

            // ### KSF FILE ###
            if ui.button("Select KSF File").clicked() {
                self.ksf_file_dialog.pick_file();
            }
            ui.label(format!("KSF file: {}", self.ksf.name));
            if let Some(e) = &self.ksf_err {
                ui.strong(e);
            } else {
                ui.strong("");
            }
            self.ksf_file_dialog.update(ui.ctx());
            if let Some(path) = self.ksf_file_dialog.take_picked() {
                match Ksf::from_file(path) {
                    Ok(ksf) => {
                        self.ksf = ksf;
                        self.session_page.load_ksf(&self.ksf);
                        self.ksf_err = None
                    }
                    Err(e) => self.ksf_err = Some(e.to_string()),
                };
            }
            ui.add_space(5.0);

            // ### CLIENT FILE ###
            if ui.button("Select Client File").clicked() {
                self.client_data_file_dialog.pick_file();
            }
            ui.label(format!(
                "Client: {} {}",
                &self.client_data.first_name, &self.client_data.last_name
            ));
            ui.label(format!("ID: {}", &self.client_data.client_id));
            ui.label(format!(
                "Session: {} TODO: make user adujustable",
                &self.client_data.session_number
            ));
            if let Some(e) = &self.client_data_err {
                ui.strong(e);
            } else {
                ui.strong("");
            }
            self.client_data_file_dialog.update(ui.ctx());
            if let Some(path) = self.client_data_file_dialog.take_picked() {
                match ClientData::from_file(path) {
                    Ok(sess_data) => {
                        self.client_data = sess_data;
                        self.client_data_err = None
                    }
                    Err(e) => self.client_data_err = Some(e.to_string()),
                };
            }

            // ### DROPDOWMS ###
            ui.group(|ui| {
                ui.label("Data Type");
                egui::ComboBox::from_id_salt("datatype")
                    .selected_text(self.session_data.data_type.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.session_data.data_type,
                            DataType::Primary,
                            "Primary",
                        );
                        ui.selectable_value(
                            &mut self.session_data.data_type,
                            DataType::Reliability,
                            "Reliability",
                        );
                    });
                ui.add_space(5.0);
                ui.label("Assessment");
                egui::ComboBox::from_id_salt("assessment")
                    .selected_text(&self.session_data.assessment)
                    .show_ui(ui, |ui| {
                        for item in self.client_data.assessments.iter() {
                            ui.selectable_value(
                                &mut self.session_data.assessment,
                                item.to_string(),
                                item,
                            );
                        }
                    });
                ui.add_space(5.0);
                ui.label("Condition");
                egui::ComboBox::from_id_salt("condition")
                    .selected_text(&self.session_data.condition)
                    .show_ui(ui, |ui| {
                        for item in self.client_data.conditions.iter() {
                            ui.selectable_value(
                                &mut self.session_data.condition,
                                item.to_string(),
                                item,
                            );
                        }
                    })
            });
        });
    }
}
