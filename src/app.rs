use crate::{
    data::{AssessmentsData, ClientData, Data, KsfData, SessionData},
    ioa::IoaPage,
    pages::{
        NewClient, NewKsf, PrepareSession, RandomServices, SessionPage, Sidebar, Timers,
        new_assessments::NewAssessments,
    },
    utils::{date_time_string, quick_file_name, windows_error_dialog},
};
use anyhow::{Context, Result};
use chrono::Local;
use egui::Visuals;
use egui_file_dialog::FileDialog;
use std::path::{Path, PathBuf};

pub const DEFAULT_ROOT_DIRECTORY_NAME: &'static str = "DataProClients";
pub const CLIENT_DATA_FILE_NAME: &'static str = "client_data.txt";
pub const ASSESSMENTS_FILE_NAME: &'static str = "assessments.txt";
pub const SESSION_DATA_FOLDER_NAME: &'static str = "Session Records";
pub const IOA_DATA_FOLDER_NAME: &'static str = "IOA Data";
pub const NO_CLIENT_MESSAGE: &'static str = "no client loaded";
const STARTING_ZOOM: f32 = 1.5;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Page {
    PrepareSession,
    RunSession,
    Ioa,
    CreateClient,
    CreateKsf,
    CreateAssessments,
}

pub struct DisplayInfo {
    pub active_page: Page,
    pub timers_open: bool,
    pub random_open: bool,
    pub sidebar_open: bool,
    pub zoom: f32,
}

impl DisplayInfo {
    pub fn go_to_prep_session(&mut self) {
        self.active_page = Page::PrepareSession;
        self.sidebar_open = true;
    }

    pub fn go_to_run_session(&mut self) {
        self.active_page = Page::RunSession;
        self.sidebar_open = false;
        self.timers_open = false;
        self.random_open = false;
    }

    pub fn go_to_ioa(&mut self) {
        self.active_page = Page::Ioa;
        self.sidebar_open = false;
    }

    pub fn go_to_new_client(&mut self) {
        self.active_page = Page::CreateClient;
        self.sidebar_open = false;
    }

    pub fn go_to_new_assessments(&mut self) {
        self.active_page = Page::CreateAssessments;
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

    pub pick_client_folder: FileDialog,
    pub pick_ksf: FileDialog,

    pub randomness_page: RandomServices,
    pub timers: Timers,

    pub prep_session: PrepareSession,
    pub session_page: SessionPage,

    pub ioa_page: IoaPage,
    pub new_client_page: NewClient,
    pub new_ksf_page: NewKsf,
    pub new_assessments_page: NewAssessments,
}

impl Default for DataPro {
    fn default() -> Self {
        let mut root_directory = std::env::current_dir().unwrap();
        root_directory.push(DEFAULT_ROOT_DIRECTORY_NAME);
        Self {
            data: Data {
                client: ClientData::default(),
                session: SessionData::default(),
                assessments: AssessmentsData::default(),
                ksf: KsfData::default(),
                ksf_name: String::new(),
            },
            display_info: DisplayInfo {
                active_page: Page::PrepareSession,
                timers_open: false,
                random_open: false,
                sidebar_open: true,
                zoom: STARTING_ZOOM,
            },

            pick_root_directory: FileDialog::default()
                .initial_directory(DEFAULT_ROOT_DIRECTORY_NAME.into()),
            root_directory,

            pick_client_folder: FileDialog::default()
                .initial_directory(DEFAULT_ROOT_DIRECTORY_NAME.into()),
            pick_ksf: FileDialog::default().initial_directory(DEFAULT_ROOT_DIRECTORY_NAME.into()),

            randomness_page: RandomServices::default(),
            timers: Timers::default(),

            prep_session: PrepareSession::default(),
            session_page: SessionPage::new(),

            ioa_page: IoaPage::default(),
            new_client_page: NewClient::default(),
            new_ksf_page: NewKsf::default(),
            new_assessments_page: NewAssessments::default(),
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
        !self.data.client.id.is_empty()
    }

    pub fn ksf_loaded(&self) -> bool {
        !self.data.ksf_name.is_empty()
    }

    pub fn assessment_chosen(&self) -> bool {
        !self.data.session.assessment.is_empty()
    }

    pub fn condition_chosen(&self) -> bool {
        !self.data.session.condition.is_empty()
    }

    /// Path to the client data file, if one is available.
    pub fn client_data_file_path(&self) -> Result<PathBuf> {
        if !self.client_loaded() {
            return Err(anyhow::anyhow!(
                "cannot find client data file because no client is selected"
            ));
        }
        let path = Path::new(&self.root_directory)
            .join(&self.data.client.id.to_string())
            .join(CLIENT_DATA_FILE_NAME);
        Ok(path.to_path_buf())
    }

    pub fn overwrite_client_data_file(&self) -> Result<()> {
        std::fs::write(
            self.client_data_file_path()?,
            &self
                .data
                .client
                .to_json()
                .with_context(|| "failed to create json version of client data file")?,
        )
        .with_context(|| "while attempting to overwrite client data file")?;

        Ok(())
    }

    pub fn client_session_data_path(&self) -> Result<PathBuf> {
        if !self.client_loaded() {
            return Err(anyhow::anyhow!(
                "cannot find {} folder because {}",
                SESSION_DATA_FOLDER_NAME,
                NO_CLIENT_MESSAGE
            ));
        }
        let path = Path::new(&self.root_directory)
            .join(&self.data.client.id.to_string())
            .join(SESSION_DATA_FOLDER_NAME);
        Ok(path.to_path_buf())
    }

    pub fn client_ioa_data_path(&self) -> Result<PathBuf> {
        if !self.client_loaded() {
            return Err(anyhow::anyhow!(
                "cannot find {} folder because {}",
                IOA_DATA_FOLDER_NAME,
                NO_CLIENT_MESSAGE
            ));
        }
        let path = Path::new(&self.root_directory)
            .join(&self.data.client.id.to_string())
            .join(IOA_DATA_FOLDER_NAME);
        Ok(path.to_path_buf())
    }

    pub fn load_ksf(&mut self, path: &PathBuf) {
        match KsfData::from_file(&path) {
            Ok(ksf) => {
                self.data.ksf = ksf;
                self.data.ksf_name = quick_file_name(&path).to_string();
            }
            Err(e) => {
                self.data.ksf = KsfData::default();
                self.data.ksf_name.clear();
                windows_error_dialog(e);
            }
        };
    }

    pub fn load_client_file(&mut self, path: &PathBuf) {
        match ClientData::from_file(&Path::new(path).join(CLIENT_DATA_FILE_NAME))
            .context("failure while loading client file")
        {
            Ok(client) => {
                self.data.client = client;
                self.data.client.current_session += 1; // We are always one session ahead of the last saved value
                match AssessmentsData::from_file(&Path::new(path).join(ASSESSMENTS_FILE_NAME))
                    .context("failure while loading assessments")
                {
                    Ok(a) => self.data.assessments = a,
                    Err(e) => {
                        windows_error_dialog(e);
                        self.data.assessments = AssessmentsData::default();
                    }
                };
                self.data.ksf = KsfData::default();
                self.data.ksf_name.clear();
                self.data.session.assessment.clear();
                self.data.session.condition.clear();
            }
            Err(e) => {
                self.data.client = ClientData::default();
                windows_error_dialog(e);
            }
        };
    }
}

impl eframe::App for DataPro {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // ### Windows ###
        self.timers.view(ui, &mut self.display_info.timers_open);
        RandomServices::view(self, ui);

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
            Sidebar::view(self, ui);
        };

        // ### Main Panel ###
        match self.display_info.active_page {
            Page::RunSession => SessionPage::view(self, ui),
            Page::Ioa => IoaPage::view(self, ui),
            Page::PrepareSession => PrepareSession::view(self, ui),
            Page::CreateClient => NewClient::view(self, ui),
            Page::CreateKsf => NewKsf::view(self, ui),
            Page::CreateAssessments => NewAssessments::view(self, ui),
        }
    }
}
