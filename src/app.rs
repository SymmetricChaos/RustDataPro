use crate::{data_tracking_page::Session, ksf::Ksf, pages::Page, randomness_page::RandomServices, timers_page::Timers, utils::date_time_string};
use chrono::Local;
use egui::{RichText, warn_if_debug_build, widgets};
use egui_file_dialog::FileDialog;

pub struct TemplateApp {
    active_page: Page,

    file_dialog: FileDialog,
    ksf_name: Option<String>,
    ksf: Ksf,
    file_err_string: Option<String>,

    randomness_page: RandomServices,
    data_tracking_page: Session,
    timer_page: Timers,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            active_page: Page::About,

            file_dialog: FileDialog::new(),
            ksf_name: None,
            ksf: Ksf::new(),
            file_err_string: None,

            randomness_page: RandomServices::default(),
            data_tracking_page: Session::default(),
            timer_page: Timers::default(),
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
        egui::Panel::top("top_panel").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                widgets::global_theme_preference_switch(ui);
                ui.separator();

                if ui.button("About").clicked() {
                    self.active_page = Page::About;
                }

                if ui.button("Randomness").clicked() {
                    self.active_page = Page::Randomness;
                }

                if ui.button("Timers").clicked() {
                    self.active_page = Page::Timers;
                }

                if ui.button("Data Tracking").clicked() {
                    self.data_tracking_page.load_ksf(&self.ksf);
                    self.active_page = Page::DataTracking;
                }
            });
        });

        match self.active_page {
            Page::About => (),
            Page::Randomness => self.randomness_page.view(ui),
            Page::DataTracking => self.data_tracking_page.view(ui),
            Page::Timers => self.timer_page.view(ui),
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
            ui.label(
                RichText::new("Data Collection and Management")
                    .heading()
                    .strong(),
            );

            ui.request_repaint_after_secs(5.0);
            ui.label(format!(
                "The Current Date/Time is {}", date_time_string(Local::now()),
            ));

            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);

            if ui.button("Select File").clicked() {
                self.file_dialog.pick_file();
            }

            if let Some(n) = &self.ksf_name {
                ui.label(format!("Picked file: {}", n));
            }
            if let Some(e) = &self.file_err_string {
                ui.strong(e);
            }

            self.file_dialog.update(ui.ctx());
            if let Some(path) = self.file_dialog.take_picked() {
                if path.extension().unwrap().to_str().unwrap() != "txt" {
                    self.file_err_string = Some(String::from("KSF files must have extension .txt"));
                } else {
                    match Ksf::from_file(path.to_str().unwrap()) {
                        Ok(ksf) => {
                            self.ksf_name =
                                Some(path.file_name().unwrap().to_str().unwrap().to_string());
                            self.ksf = ksf;
                            self.file_err_string = None
                        }
                        Err(e) => self.file_err_string = Some(e.to_string()),
                    };
                }
            }
        });
    }
}
