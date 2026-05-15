use crate::{
    controls::{key_label, KEY_BINDINGS, MOUSE_BINDINGS},
    user::Settings,
    utils::add_row,
};
use bevy_egui::egui::{self, Color32, Frame, Label, RichText, SliderClamping, Stroke};

pub(crate) fn settings_content(ui: &mut egui::Ui, settings: &mut Settings) {
    let volume_slider = egui::Slider::new(&mut settings.volume.0, 0..=100)
        .show_value(false)
        .clamping(SliderClamping::Always);
    add_row("Volume", volume_slider, ui);
}

pub(crate) fn controls_content(ui: &mut egui::Ui, show_heading: bool) {
    ui.set_width(ui.available_width());

    if show_heading {
        ui.heading("Controls & Key Bindings");
        ui.add_space(22.);
    }

    controls_section(
        ui,
        "Keyboard",
        KEY_BINDINGS
            .iter()
            .map(|binding| (key_label(binding.key_code), binding.label)),
    );

    ui.add_space(18.);
    controls_section(
        ui,
        "Mouse Controls In-Game",
        MOUSE_BINDINGS
            .iter()
            .map(|binding| (binding.input, binding.description)),
    );
}

fn controls_section<'a>(
    ui: &mut egui::Ui,
    title: &str,
    rows: impl IntoIterator<Item = (&'a str, &'a str)>,
) {
    Frame::new()
        .fill(Color32::from_white_alpha(10))
        .stroke(Stroke::new(1., Color32::from_white_alpha(24)))
        .inner_margin(egui::Margin::symmetric(22, 18))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.label(
                RichText::new(title)
                    .size(24.)
                    .strong()
                    .color(Color32::from_gray(220)),
            );
            ui.add_space(12.);

            for (index, (input, description)) in rows.into_iter().enumerate() {
                control_row(ui, index, input, description);
            }
        });
}

fn control_row(ui: &mut egui::Ui, index: usize, input: &str, description: &str) {
    let row_fill = match index % 2 {
        0 => Color32::from_white_alpha(12),
        _ => Color32::from_white_alpha(6),
    };

    Frame::new()
        .fill(row_fill)
        .inner_margin(egui::Margin::symmetric(14, 9))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            let key_width = (ui.available_width() * 0.26).clamp(150., 240.);
            ui.horizontal(|ui| {
                ui.add_sized(
                    [key_width, 26.],
                    Label::new(
                        RichText::new(input)
                            .monospace()
                            .strong()
                            .color(Color32::from_gray(225)),
                    ),
                );
                ui.add_sized(
                    [ui.available_width(), 26.],
                    Label::new(RichText::new(description).color(Color32::from_gray(185))),
                );
            });
        });
    ui.add_space(6.);
}
