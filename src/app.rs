use crate::{pages::Page, randomness_page::RandomnessPage};
use chrono::{Datelike, Local, Timelike};
use eframe::App;
use egui::{
    CentralPanel, Context, FontData, FontDefinitions, FontFamily, RichText, SidePanel,
    TopBottomPanel, warn_if_debug_build, widgets,
};
use std::sync::Arc;

fn load_font(name: &str, family: &FontFamily, font_data: FontData, font_def: &mut FontDefinitions) {
    font_def.font_data.insert(name.into(), Arc::new(font_data));
    font_def.families.get_mut(family).unwrap().push(name.into());
}

pub struct RustDataPro {
    active_page: Page,

    randomness_page: RandomnessPage,
}

impl RustDataPro {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut font_def = FontDefinitions::default();
        // Noto fonts to get wide coverage, more can be added if needed
        load_font(
            "NotoMono",
            &FontFamily::Monospace,
            FontData::from_static(include_bytes!("../NotoSansMono-Regular.ttf")),
            &mut font_def,
        );
        load_font(
            "NotoSans",
            &FontFamily::Proportional,
            FontData::from_static(include_bytes!("../NotoSans-Regular.ttf")),
            &mut font_def,
        );
        load_font(
            "NotoSymbols",
            &FontFamily::Proportional,
            FontData::from_static(include_bytes!("../NotoSansSymbols-Regular.ttf")),
            &mut font_def,
        );
        load_font(
            "NotoSymbols2",
            &FontFamily::Proportional,
            FontData::from_static(include_bytes!("../NotoSansSymbols2-Regular.ttf")),
            &mut font_def,
        );
        load_font(
            "NotoMath",
            &FontFamily::Proportional,
            FontData::from_static(include_bytes!("../NotoSansMath-Regular.ttf")),
            &mut font_def,
        );
        cc.egui_ctx.set_fonts(font_def);

        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        Self::default()
    }

    fn randomness_page(&mut self, ctx: &Context) {
        self.randomness_page.view(&ctx)
    }

    fn about_page(&mut self, ctx: &Context) {
        SidePanel::left("about_display_panel")
            .default_width(500.0)
            .min_width(200.0)
            .show(ctx, |ui| {
                warn_if_debug_build(ui);
                let hello = RichText::new("Welcome to RustDataPro!").strong();
                ui.label(hello);
                ui.add_space(20.0);
                // ui.hyperlink_to(
                //     "source code",
                //     "https://github.com/SymmetricChaos/crypto-gui",
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
            });
        CentralPanel::default().show(ctx, |ui| {
            ui.label(
                RichText::new("Data Collection and Management")
                    .heading()
                    .strong(),
            );
            let dt = Local::now();

            ui.label(format!(
                "The Current Date/Time is {} {}/{}/{} {:02}:{:02}:{:02}",
                dt.weekday(),
                dt.month(),
                dt.day(),
                dt.year(),
                dt.hour(),
                dt.minute(),
                dt.second(),
            ));
        });
    }
}

impl Default for RustDataPro {
    fn default() -> Self {
        Self {
            active_page: Page::About,
            randomness_page: RandomnessPage::default(),
        }
    }
}

impl App for RustDataPro {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                widgets::global_theme_preference_switch(ui);
                ui.separator();

                if ui.button("About").clicked() {
                    self.active_page = Page::About;
                }

                if ui.button("Randomness").clicked() {
                    self.active_page = Page::Randomness;
                }
            });
        });

        match self.active_page {
            Page::About => self.about_page(ctx),
            Page::Randomness => self.randomness_page(ctx),
        }
    }
}
