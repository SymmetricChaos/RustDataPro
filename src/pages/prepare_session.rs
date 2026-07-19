use crate::{
    app::{DataPro, SESSION_DATA_FOLDER_NAME},
    data::DataType,
    utils::{DataProUiElements, windows_error_dialog},
};
use egui::{Color32, TextStyle};
use egui_file_dialog::FileDialog;
use std::path::Path;

pub struct PrepareSession {
    pub can_start_session: bool,
}

impl Default for PrepareSession {
    fn default() -> Self {
        Self {
            can_start_session: true,
        }
    }
}

impl PrepareSession {
    pub fn view(app: &mut DataPro, ui: &mut egui::Ui) {
        app.pick_ksf.update(ui.ctx());
        if let Some(path) = app.pick_ksf.take_picked() {
            app.load_ksf_file(&path);
        }

        app.pick_client_folder.update(ui.ctx());
        if let Some(path) = app.pick_client_folder.take_picked() {
            app.load_client_file(&path.clone());
            app.pick_ksf = FileDialog::new().initial_directory(path.clone());
            let ioa = Path::new(&path).join(SESSION_DATA_FOLDER_NAME);
            app.ioa_page.file_dialog = FileDialog::new().initial_directory(ioa);
        }

        app.prep_session.can_start_session = app.client_loaded() && app.ksf_loaded();

        egui::CentralPanel::default().show(ui, |ui| {
            ui.add_space(15.0);
            if ui.large_button("Select Client").clicked() {
                app.pick_client_folder.pick_directory();
            }
            ui.add_space(5.0);

            ui.add_enabled_ui(app.client_loaded(), |ui| {
                if ui
                    .large_button("Select KSF")
                    .on_disabled_hover_text("no client selected")
                    .clicked()
                {
                    app.pick_ksf.pick_file();
                }
                ui.label(format!("KSF: {}", app.data.ksf_name));
                ui.add_space(5.0);
            });

            ui.add_enabled_ui(app.client_loaded(), |ui| {
                egui::Grid::new("client_and_session_info_grid")
                    .min_col_width(150.0)
                    .min_row_height(22.0)
                    .show(ui, |ui| {
                        // For reasons of client privacy this is best not display and perhaps best not stored in the client file at all
                        // ui.monospace("Client Name");
                        // ui.monospace(&app.data.client.name);
                        // ui.end_row();

                        ui.monospace("Client ID");
                        ui.add(
                            egui::TextEdit::singleline(&mut app.data.client.id.to_string())
                                .font(TextStyle::Monospace)
                                .interactive(false),
                        );
                        ui.end_row();

                        ui.monospace("Location");
                        if ui
                            .text_edit_singleline(&mut app.data.client.location)
                            .lost_focus()
                        {
                            if let Err(e) = app.overwrite_client_data_file() {
                                windows_error_dialog(e)
                            }
                        }
                        ui.end_row();

                        ui.monospace("Date of Admission");
                        match app.data.client.days_since_admission() {
                            Ok(n) => {
                                if n.is_negative() {
                                    ui.add(
                                        egui::TextEdit::singleline(&mut format!("{n} days ago"))
                                            .font(TextStyle::Monospace)
                                            .text_color(Color32::RED)
                                            .interactive(false),
                                    );
                                app.prep_session.can_start_session = false;
                                } else {
                                    ui.add(
                                        egui::TextEdit::singleline(&mut format!("{n} days ago"))
                                            .font(TextStyle::Monospace)
                                            .interactive(false),
                                    );
                                }
                            }
                            Err(_e) => {
                                ui.add(
                                    egui::TextEdit::singleline(&mut format!("ERROR"))
                                        .font(TextStyle::Monospace)
                                        .text_color(Color32::RED)
                                        .interactive(false),
                                )
                                .on_hover_text(format!("Date of Admission cannot be {} because it is in invalid date\nformat date as YYYY-MM-DD",app.data.client.date_of_admission));
                                app.prep_session.can_start_session = false;
                            }
                        }
                        ui.end_row();

                        ui.monospace("Session Number");
                        if ui
                            .add(egui::DragValue::new(&mut app.data.client.current_session))
                            .lost_focus()
                        {
                            if let Err(e) = app.overwrite_client_data_file() {
                                windows_error_dialog(e)
                            }
                        }
                        ui.end_row();

                        ui.monospace("Case Manager");
                        ui.add(
                            egui::TextEdit::singleline(&mut app.data.client.case_manager)
                                .font(TextStyle::Monospace)
                                .interactive(false),
                        );
                        ui.end_row();

                        ui.monospace("Primary Therapist");
                        ui.add(
                            egui::TextEdit::singleline(&mut app.data.client.primary_therapist)
                                .font(TextStyle::Monospace)
                                .interactive(false),
                        );
                        ui.end_row();

                        ui.monospace("Session Therapist");
                        ui.text_edit_singleline(&mut app.data.session.therapist);
                        ui.end_row();

                        ui.monospace("Data Collector");
                        ui.text_edit_singleline(&mut app.data.session.data_collector);
                        ui.end_row();

                        ui.monospace("Data Type");
                        egui::ComboBox::from_id_salt("datatype")
                            .selected_text(app.data.session.data_type.to_string())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut app.data.session.data_type,
                                    DataType::Primary,
                                    "Primary",
                                );
                                ui.selectable_value(
                                    &mut app.data.session.data_type,
                                    DataType::Reliability,
                                    "Reliability",
                                );
                            });
                        ui.end_row();

                        ui.monospace("Assessment");
                        egui::ComboBox::from_id_salt("assessment")
                            .selected_text(&app.data.session.assessment)
                            .show_ui(ui, |ui| {
                                for item in app.data.client.assessments.iter() {
                                    ui.selectable_value(
                                        &mut app.data.session.assessment,
                                        item.to_string(),
                                        item,
                                    );
                                }
                            });
                        ui.end_row();

                        ui.monospace("Condition");
                        egui::ComboBox::from_id_salt("condition")
                            .selected_text(&app.data.session.condition)
                            .show_ui(ui, |ui| {
                                for item in app.data.client.conditions.iter() {
                                    ui.selectable_value(
                                        &mut app.data.session.condition,
                                        item.to_string(),
                                        item,
                                    );
                                }
                            });
                        ui.end_row();
                    });
                ui.add_space(10.0);
            });

            ui.horizontal(|ui| {
                ui.add_enabled(
                    app.session_page.limit_session_length,
                    egui::DragValue::new(&mut app.session_page.maximum_session_length)
                        .range(0.0..=100_000.0),
                );
                ui.checkbox(
                    &mut app.session_page.limit_session_length,
                    "Limit Session Length",
                );
            });
            ui.add_space(10.0);



            ui.add_enabled_ui(app.prep_session.can_start_session, |ui| {
                if ui
                    .large_green_button("BEGIN SESSION")
                    .on_disabled_hover_text("there files that need to be loaded or errors that must be resolved")
                    .clicked()
                {
                    // Update the client file with any changes
                    // This is only relevant if the user changes a client field and then immediately clicks BEGIN SESSION
                    // If they do anything else the file will update when they switch pages
                    if let Err(e) = app.overwrite_client_data_file() {
                        windows_error_dialog(e)
                    } else {
                        // Load the data and switch pages.
                        app.session_page.load_ksf(&app.data);
                        app.timers.pause_all_timers();
                        app.display_info.go_to_run_session();
                    }
                }
            })
        });
    }
}
