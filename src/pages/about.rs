use crate::{app::DataPro, data::DataType, utils::DataProUiElements};

pub struct About {}

impl About {
    pub fn view(app: &mut DataPro, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.add_space(15.0);

            app.ksf_file_dialog.update(ui.ctx());
            if let Some(path) = app.ksf_file_dialog.take_picked() {
                app.load_ksf_file(path);
            }
            app.client_data_file_dialog.update(ui.ctx());
            if let Some(path) = app.client_data_file_dialog.take_picked() {
                app.load_client_file(path);
            }

            if ui.large_button("Select KSF").clicked() {
                app.ksf_file_dialog.pick_file();
            }
            ui.label(format!("KSF: {}", app.data.ksf.name));
            ui.strong(&app.ksf_err);

            if ui.large_button("Select Client File").clicked() {
                app.client_data_file_dialog.pick_file();
            }
            ui.strong(&app.client_data_err);
            ui.add_enabled_ui(true, |ui| {
                egui::Grid::new("client_and_session_info_grid").show(ui, |ui| {
                    ui.monospace("Client");
                    ui.monospace(&app.data.client.name);
                    ui.end_row();

                    ui.monospace("ID");
                    ui.monospace(&app.data.client.client_id);
                    ui.end_row();

                    ui.monospace("Case Manager");
                    ui.monospace(&app.data.client.case_manager);
                    ui.end_row();

                    ui.monospace("Primary Therapist");
                    ui.monospace(&app.data.client.primary_therapist);
                    ui.end_row();

                    ui.monospace("Session");
                    ui.add(egui::DragValue::new(&mut app.data.client.session_number));
                    ui.end_row();

                    ui.monospace("Therapist");
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

            if ui.large_green_button("BEGIN SESSION").clicked() {
                app.session_page.load_ksf(&app.data);
                app.display_info.go_to_session();
                // app.timers.stop_all_timers();
            }
        });
    }
}
