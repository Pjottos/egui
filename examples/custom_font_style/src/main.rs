#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{FontFamily, FontId, RichText, TextStyle};

#[inline]
fn heading2() -> TextStyle {
    TextStyle::Name("Heading2".into())
}

#[inline]
fn heading3() -> TextStyle {
    TextStyle::Name("ContextHeading".into())
}

fn configure_text_styles(ctx: &mut egui::Context) {
    use FontFamily::Proportional;

    ctx.style_mut().text_styles = [
        (TextStyle::Heading, FontId::new(30.0, Proportional)),
        (heading2(), FontId::new(25.0, Proportional)),
        (heading3(), FontId::new(23.0, Proportional)),
        (TextStyle::Body, FontId::new(18.0, Proportional)),
        (TextStyle::Monospace, FontId::new(14.0, Proportional)),
        (TextStyle::Button, FontId::new(14.0, Proportional)),
        (TextStyle::Small, FontId::new(10.0, Proportional)),
    ]
    .into();
}

fn content(ui: &mut egui::Ui<'_>) {
    ui.heading("Top Heading");
    ui.add_space(5.);
    ui.label(LOREM_IPSUM);
    ui.add_space(15.);
    ui.label(RichText::new("Sub Heading").text_style(heading2()).strong());
    ui.label(LOREM_IPSUM);
    ui.add_space(15.);
    ui.label(RichText::new("Context").text_style(heading3()).strong());
    ui.add_space(5.);
    ui.label(LOREM_IPSUM);
}

struct MyApp;

impl MyApp {
    fn new(cc: &mut eframe::CreationContext<'_, '_>) -> Self {
        configure_text_styles(cc.egui_ctx);
        Self
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &mut egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, content);
    }
}

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "egui example: global font style",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    );
}

pub const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
