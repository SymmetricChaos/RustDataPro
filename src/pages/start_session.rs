use crate::{app::DataPro, data::DataType, utils::DataProUiElements};
use egui_file_dialog::FileDialog;

pub struct StartSession {}

impl StartSession {
    pub fn view(app: &mut DataPro, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.add_space(15.0);

            app.pick_ksf.update(ui.ctx());
            if let Some(path) = app.pick_ksf.take_picked() {
                app.load_ksf_file(&path);
            }

            app.pick_client_folder.update(ui.ctx());
            if let Some(path) = app.pick_client_folder.take_picked() {
                app.load_client_file(&path);
                app.pick_ksf = FileDialog::new().initial_directory(path);
            }

            if ui.large_button("Select Client").clicked() {
                app.pick_client_folder.pick_directory();
            }
            ui.strong(&app.client_data_err);
            ui.add_space(5.0);

            ui.add_enabled_ui(app.data.client.id != "None", |ui| {
                if ui
                    .large_button("Select KSF")
                    .on_disabled_hover_text("no client selected")
                    .clicked()
                {
                    app.pick_ksf.pick_file();
                }

                ui.label(format!("KSF: {}", app.data.ksf_name));
                ui.strong(&app.ksf_err);
                ui.add_space(5.0);
            });

            ui.add_enabled_ui(true, |ui| {
                egui::Grid::new("client_and_session_info_grid")
                    .min_col_width(150.0)
                    .show(ui, |ui| {
                        // For reasons of client privacy this is best not display and perhaps best not stored in the client file at all
                        // ui.monospace("Client Name");
                        // ui.monospace(&app.data.client.name);
                        // ui.end_row();

                        ui.monospace("Client ID");
                        ui.monospace(&app.data.client.id);
                        ui.end_row();

                        ui.monospace("Location");
                        ui.text_edit_singleline(&mut app.data.client.location);
                        ui.end_row();

                        ui.monospace("Date of Admission");
                        ui.monospace(&format!(
                            "{} ({} days ago)",
                            app.data.client.date_of_admission,
                            app.data.client.days_since_admission()
                        ));
                        ui.end_row();

                        ui.monospace("Session Number");
                        if ui
                            .add(egui::DragValue::new(&mut app.data.client.current_session))
                            .changed()
                        {
                            // let path = Path::new(&app.root_directory);
                            // path.join(&app.data.client.id.to_string());
                            // path.join(CLIENT_DATA_FILE_NAME);
                            // std::fs::write(
                            //     path,
                            //     &app.data.client.to_json().expect("failed to create json"),
                            // )
                            // .expect("failed to save update client file");
                        }
                        ui.end_row();

                        ui.monospace("Case Manager");
                        ui.monospace(&app.data.client.case_manager);
                        ui.end_row();

                        ui.monospace("Primary Therapist");
                        ui.monospace(&app.data.client.primary_therapist);
                        ui.end_row();

                        ui.monospace("Session Therapist");
                        ui.text_edit_singleline(&mut app.data.session.therapist);
                        ui.end_row();

                        ui.monospace("Data Collector");
                        ui.text_edit_singleline(&mut app.data.session.data_collector);
                        ui.end_row();
                    });
            });

            // ### DROPDOWMS ###
            ui.group(|ui| {
                ui.label("Data Type");
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
                ui.add_space(5.0);
                ui.label("Assessment");
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
                ui.add_space(5.0);
                ui.label("Condition");
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
                    })
            });
            ui.add_space(10.0);

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

            let can_start_session = app.data.client.id != "None" && app.data.ksf_name != "";
            ui.add_enabled_ui(can_start_session, |ui| {
                if ui
                    .large_green_button("BEGIN SESSION")
                    .on_disabled_hover_text("both client and KSF must be selected")
                    .clicked()
                {
                    app.session_page.load_ksf(&app.data);
                    app.display_info.go_to_session();
                    app.timers.pause_all_timers();
                }
            })
        });
    }
}
