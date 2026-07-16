use crate::{
    data::{ClientData, Data, KsfData, SessionData},
    pages::{self, RandomServices, Timers, new_client::NewClient},
    reliability::ReliabilityPage,
    utils::{date_time_string, quick_file_name},
};
use chrono::Local;
use egui::Visuals;
use egui_file_dialog::FileDialog;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Page {
    About,
    Session,
    Reliability,
    CreateClient,
}

pub struct DisplayInfo {
    pub active_page: Page,
    pub timers_open: bool,
    pub random_open: bool,
    pub sidebar_open: bool,
    pub zoom: f32,
    pub ksf_name: String,
}

impl DisplayInfo {
    pub fn go_to_about(&mut self) {
        self.active_page = Page::About;
        self.sidebar_open = true;
    }

    pub fn go_to_session(&mut self) {
        self.active_page = Page::Session;
        self.sidebar_open = false;
        self.timers_open = false;
        self.random_open = false;
    }

    pub fn go_to_reliability(&mut self) {
        self.active_page = Page::Reliability;
        self.sidebar_open = false;
    }

    pub fn go_to_new_client(&mut self) {
        self.active_page = Page::CreateClient;
        self.sidebar_open = false;
    }

    pub fn toggle_timer_display(&mut self) {
        self.timers_open = !self.timers_open;
    }

    pub fn toggle_random_display(&mut self) {
        self.random_open = !self.random_open;
    }
}

const STARTING_ZOOM: f32 = 1.5;

pub struct DataPro {
    pub data: Data,
    pub display_info: DisplayInfo,

    pub ksf_file_dialog: FileDialog,
    pub ksf_err: String,

    pub client_data_file_dialog: FileDialog,
    pub client_data_err: String,
    pub client_data_path: Option<String>,

    pub randomness_page: RandomServices,
    pub timers: Timers,

    pub session_page: pages::SessionPage,
    pub reliability_page: ReliabilityPage,
    pub new_client_page: NewClient,
}

impl Default for DataPro {
    fn default() -> Self {
        Self {
            data: Data {
                client: ClientData::default(),
                session: SessionData::default(),
                ksf: KsfData::default(),
            },
            display_info: DisplayInfo {
                active_page: Page::About,
                timers_open: false,
                random_open: false,
                sidebar_open: true,
                zoom: STARTING_ZOOM,
                ksf_name: String::from("DEFAULT"),
            },

            ksf_file_dialog: FileDialog::default(),
            ksf_err: String::default(),

            client_data_file_dialog: FileDialog::default(),
            client_data_err: String::default(),
            client_data_path: None,

            randomness_page: RandomServices::default(),
            timers: Timers::default(),

            session_page: pages::SessionPage::new(),
            reliability_page: ReliabilityPage::default(),
            new_client_page: NewClient::default(),
        }
    }
}

impl DataPro {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(STARTING_ZOOM);
        cc.egui_ctx.set_visuals(Visuals::dark());
        Default::default()
    }

    pub fn load_ksf_file(&mut self, path: PathBuf) {
        match KsfData::from_file(&path) {
            Ok(ksf) => {
                self.data.ksf = ksf;
                self.display_info.ksf_name = quick_file_name(&path).to_string();
                self.ksf_err.clear();
            }
            Err(e) => {
                self.data.ksf = KsfData::default();
                self.display_info.ksf_name = String::from("DEFAULT");
                self.ksf_err = e.to_string();
            }
        };
    }

    pub fn load_client_file(&mut self, path: PathBuf) {
        match ClientData::from_file(&path) {
            Ok(sess_data) => {
                self.client_data_path = Some(path.as_path().to_str().unwrap().to_string());
                self.data.client = sess_data;
                self.data.client.current_session += 1; // We are always one session ahead of the last saved value
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
        // ### Windows ###
        self.timers.view(ui, &mut self.display_info.timers_open);
        self.randomness_page
            .view(ui, &mut self.display_info.random_open);

        // ### Top Bar ###
        // To go fully across it must be specified before any other panel
        // Nothing here can be interactable because we use Tab and Space as controls on the Session Page
        egui::Panel::top("top_panel").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.request_repaint_after_secs(5.0);
                ui.label(format!("{}", date_time_string(&Local::now())));
            });
        });

        // ### Sidebar ###
        // To show it must go before any other panel
        // It must be not to rendered (even if not shown) when Session is active because it may capture keypresses
        if self.display_info.sidebar_open {
            pages::Sidebar::view(self, ui);
        };

        // ### Main Panel ###
        match self.display_info.active_page {
            Page::Session => self.session_page.view(
                ui,
                &mut self.display_info,
                &mut self.data,
                &self.client_data_path,
            ),
            Page::Reliability => self.reliability_page.view(ui, &mut self.display_info),
            Page::About => pages::About::view(self, ui),
            Page::CreateClient => self.new_client_page.view(ui, &mut self.display_info),
        }
    }
}
