use crate::{
    data::{ClientData, Data, KsfData, SessionData},
    ioa::IoaPage,
    pages::{self, RandomServices, SessionPage, Timers, new_client::NewClient, new_ksf::NewKsf},
    utils::{date_time_string, quick_file_name},
};
use anyhow::Result;
use chrono::Local;
use egui::Visuals;
use egui_file_dialog::FileDialog;
use std::path::{Path, PathBuf};

pub const DEFAULT_ROOT_DIRECTORY_NAME: &'static str = "DataProClients";
pub const CLIENT_DATA_FILE_NAME: &'static str = "client_data.txt";
pub const CLIENT_SESSION_DATA_FOLDER_NAME: &'static str = "SessionRecords";
const STARTING_ZOOM: f32 = 1.5;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Page {
    BeginSession,
    Session,
    Reliability,
    CreateClient,
    CreateKsf,
}

pub struct DisplayInfo {
    pub active_page: Page,
    pub timers_open: bool,
    pub random_open: bool,
    pub sidebar_open: bool,
    pub zoom: f32,
}

impl DisplayInfo {
    pub fn go_to_begin_session(&mut self) {
        self.active_page = Page::BeginSession;
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

    pub fn go_to_new_ksf(&mut self) {
        self.active_page = Page::CreateKsf;
        self.sidebar_open = false;
    }

    pub fn toggle_timer_display(&mut self) {
        self.timers_open = !self.timers_open;
    }

    pub fn toggle_random_display(&mut self) {
        self.random_open = !self.random_open;
    }
}

pub struct DataPro {
    pub pick_root_directory: FileDialog,
    pub root_directory: PathBuf,

    pub data: Data,
    pub display_info: DisplayInfo,

    pub pick_ksf: FileDialog,
    pub ksf_err: String,

    pub pick_client_folder: FileDialog,
    pub client_data_err: String,

    pub randomness_page: RandomServices,
    pub timers: Timers,

    pub session_page: SessionPage,
    pub reliability_page: IoaPage,
    pub new_client_page: NewClient,
    pub new_ksf_page: NewKsf,
}

impl Default for DataPro {
    fn default() -> Self {
        let mut root_directory = std::env::current_dir().unwrap();
        root_directory.push(DEFAULT_ROOT_DIRECTORY_NAME);
        Self {
            data: Data {
                client: ClientData::default(),
                session: SessionData::default(),
                ksf: KsfData::default(),
                ksf_name: String::new(),
            },
            display_info: DisplayInfo {
                active_page: Page::BeginSession,
                timers_open: false,
                random_open: false,
                sidebar_open: true,
                zoom: STARTING_ZOOM,
            },

            pick_root_directory: FileDialog::default()
                .initial_directory(DEFAULT_ROOT_DIRECTORY_NAME.into()),
            root_directory,

            pick_ksf: FileDialog::default().initial_directory(DEFAULT_ROOT_DIRECTORY_NAME.into()),
            ksf_err: String::default(),

            pick_client_folder: FileDialog::default()
                .initial_directory(DEFAULT_ROOT_DIRECTORY_NAME.into()),
            client_data_err: String::default(),

            randomness_page: RandomServices::default(),
            timers: Timers::default(),

            session_page: SessionPage::new(),
            reliability_page: IoaPage::default(),
            new_client_page: NewClient::default(),
            new_ksf_page: NewKsf::default(),
        }
    }
}

impl DataPro {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(STARTING_ZOOM);
        cc.egui_ctx.set_visuals(Visuals::dark());
        Default::default()
    }

    pub fn client_loaded(&self) -> bool {
        self.data.client.id != "None"
    }

    pub fn ksf_loaded(&self) -> bool {
        self.data.ksf_name != ""
    }

    /// Path to the client data file, if one is available.
    pub fn client_data_file(&self) -> Result<PathBuf> {
        if !self.client_loaded() {
            return Err(anyhow::anyhow!("no client selected"));
        }
        let path = Path::new(&self.root_directory)
            .join(&self.data.client.id.to_string())
            .join(CLIENT_DATA_FILE_NAME);
        Ok(path.to_path_buf())
    }

    pub fn load_ksf_file(&mut self, path: &PathBuf) {
        match KsfData::from_file(&path) {
            Ok(ksf) => {
                self.data.ksf = ksf;
                self.data.ksf_name = quick_file_name(&path).to_string();
                self.ksf_err.clear();
            }
            Err(e) => {
                self.data.ksf = KsfData::default();
                self.data.ksf_name = String::from("");
                self.ksf_err = e.to_string();
            }
        };
    }

    pub fn load_client_file(&mut self, path: &PathBuf) {
        let mut path = path.clone();
        path.push(CLIENT_DATA_FILE_NAME);
        match ClientData::from_file(&path) {
            Ok(client) => {
                self.data.client = client;
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
            Page::Session => pages::session_page::SessionPage::view(self, ui),
            Page::Reliability => self.reliability_page.view(ui, &mut self.display_info),
            Page::BeginSession => pages::StartSession::view(self, ui),
            Page::CreateClient => {
                self.new_client_page
                    .view(ui, &mut self.display_info, &self.root_directory)
            }
            Page::CreateKsf => self.new_ksf_page.view(
                ui,
                &mut self.data,
                &mut self.display_info,
                &self.root_directory,
            ),
        }
    }
}
