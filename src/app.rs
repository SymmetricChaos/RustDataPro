use crate::{
    data::{ClientData, DataType, SessionData, ksf::Ksf},
    pages::Page,
    random_services::RandomServices,
    reliability::ReliabilityPage,
    session_page::SessionPage,
    timers::Timers,
    utils::{DataProUiElements, date_time_string},
};
use chrono::Local;
use egui::{RichText, SurrenderFocusOn::Never, Visuals, warn_if_debug_build};
use egui_file_dialog::FileDialog;
use std::path::PathBuf;

pub struct Data {
    pub client: ClientData,
    pub session: SessionData,
    pub ksf: Ksf,
}

pub struct DisplayInfo {
    pub active_page: Page,
    pub timers_open: bool,
    pub random_open: bool,
    pub welcome_open: bool,
}

impl DisplayInfo {
    pub fn go_to_about(&mut self) {
        self.active_page = Page::About;
        self.welcome_open = true;
    }

    pub fn go_to_session(&mut self) {
        self.active_page = Page::Session;
        self.welcome_open = false;
    }

    pub fn go_to_reliability(&mut self) {
        self.active_page = Page::Reliability;
        self.welcome_open = false;
    }

    pub fn toggle_timer_display(&mut self) {
        self.timers_open = !self.timers_open;
    }

    pub fn toggle_random_display(&mut self) {
        self.random_open = !self.random_open;
    }
}

pub struct DataPro {
    data: Data,
    display_info: DisplayInfo,

    ksf_file_dialog: FileDialog,
    ksf_err: String,

    client_data_file_dialog: FileDialog,
    client_data_err: String,
    client_data_path: Option<String>,

    randomness_page: RandomServices,
    timers: Timers,

    session_page: SessionPage,
    reliability_page: ReliabilityPage,
}

impl Default for DataPro {
    fn default() -> Self {
        Self {
            data: Data {
                client: ClientData::default(),
                session: SessionData::default(),
                ksf: Ksf::default(),
            },
            display_info: DisplayInfo {
                active_page: Page::About,
                timers_open: false,
                random_open: false,
                welcome_open: true,
            },

            ksf_file_dialog: FileDialog::new(),
            ksf_err: String::new(),

            client_data_file_dialog: FileDialog::new(),
            client_data_err: String::new(),
            client_data_path: None,

            randomness_page: RandomServices::default(),
            timers: Timers::default(),

            session_page: SessionPage::new(),
            reliability_page: ReliabilityPage::default(),
        }
    }
}

impl DataPro {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.75);
        cc.egui_ctx.set_visuals(Visuals::dark());
        cc.egui_ctx
            .options_mut(|options| options.input_options.surrender_focus_on = Never);
        Default::default()
    }

    fn load_ksf_file(&mut self, path: PathBuf) {
        match Ksf::from_file(path) {
            Ok(ksf) => {
                self.data.ksf = ksf;
                self.session_page.load_ksf(&self.data.ksf);
                self.ksf_err.clear();
            }
            Err(e) => {
                self.ksf_err = e.to_string();
                self.data.ksf = Ksf::default()
            }
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
            Err(e) => {
                self.client_data_err = e.to_string();
                self.data.client = ClientData::default()
            }
        };
    }
}

impl eframe::App for DataPro {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top_panel").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.request_repaint_after_secs(5.0);
                ui.label(format!("{}", date_time_string(Local::now())));
            });
        });

        self.timers.view(ui, &mut self.display_info.timers_open);
        self.randomness_page
            .view(ui, &mut self.display_info.random_open);

        if self.display_info.welcome_open {
            egui::Panel::left("welcome_panel")
                .default_size(200.0)
                .min_size(200.0)
                .show(ui, |ui| {
                    warn_if_debug_build(ui);
                    let hello = RichText::new("Welcome to RustDataPro!").strong();
                    ui.label(hello);
                    ui.add_space(10.0);
                    // ui.hyperlink_to(
                    //     "view the source code",
                    //     "https://github.com/SymmetricChaos/RustDataPro",
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

                    ui.add_space(20.0);
                    ui.label("Other Useful Functionality");
                    ui.add_space(5.0);
                    if ui.large_button("Randomness").clicked() {
                        self.display_info.toggle_random_display();
                    }

                    ui.add_space(5.0);
                    if ui.large_button("Timers").clicked() {
                        self.display_info.toggle_timer_display();
                    }

                    ui.add_space(5.0);
                    if ui.large_button("Reliability").clicked() {
                        self.display_info.go_to_reliability();
                    }
                });
        };

        match self.display_info.active_page {
            Page::Session => self.session_page.view(
                ui,
                &mut self.display_info,
                &mut self.data,
                &self.client_data_path,
            ),
            Page::Reliability => self.reliability_page.view(ui, &mut self.display_info),
            Page::About => {
                egui::CentralPanel::default().show(ui, |ui| {
                    ui.add_space(15.0);

                    self.ksf_file_dialog.update(ui.ctx());
                    if let Some(path) = self.ksf_file_dialog.take_picked() {
                        self.load_ksf_file(path);
                    }
                    self.client_data_file_dialog.update(ui.ctx());
                    if let Some(path) = self.client_data_file_dialog.take_picked() {
                        self.load_client_file(path);
                    }

                    if ui.large_button("Select KSF").clicked() {
                        self.ksf_file_dialog.pick_file();
                    }
                    ui.label(format!("KSF: {}", self.data.ksf.name));
                    ui.strong(&self.ksf_err);

                    // ui.add_enabled_ui(true, |ui| {
                    //     egui::Grid::new("ksf_info_grid").show(ui, |ui| {
                    //     });
                    // });

                    if ui.large_button("Select Client File").clicked() {
                        self.client_data_file_dialog.pick_file();
                    }
                    ui.strong(&self.client_data_err);
                    ui.add_enabled_ui(true, |ui| {
                        egui::Grid::new("client_and_session_info_grid").show(ui, |ui| {
                            ui.monospace("Client");
                            ui.monospace(&self.data.client.name);
                            ui.end_row();

                            ui.monospace("ID");
                            ui.monospace(&self.data.client.client_id);
                            ui.end_row();

                            ui.monospace("Case Manager");
                            ui.monospace(&self.data.client.case_manager);
                            ui.end_row();

                            ui.monospace("Primary Therapist");
                            ui.monospace(&self.data.client.primary_therapist);
                            ui.end_row();

                            ui.monospace("Session");
                            ui.add(egui::DragValue::new(&mut self.data.client.session_number));
                            ui.end_row();

                            ui.monospace("Therapist");
                            ui.text_edit_singleline(&mut self.data.session.therapist);
                            ui.end_row();

                            ui.monospace("Data Collector");
                            ui.text_edit_singleline(&mut self.data.session.data_collector);
                            ui.end_row();
                        });
                    });

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
                    ui.add_space(10.0);

                    if ui.large_green_button("BEGIN SESSION").clicked() {
                        self.display_info.go_to_session();
                    }
                });
            }
        }
    }
}
