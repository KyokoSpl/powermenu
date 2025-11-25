use std::process::Command;

use eframe::egui;
use egui_material3::theme::{
    get_global_theme, load_fonts, load_themes, setup_local_theme, update_window_background,
    ThemeMode,
};

mod logic;

struct PowerMenuApp {
    // Animation states for 4 buttons: Shutdown, Lock, Suspend, Reboot
    hover_scales: [f32; 4],
}

impl PowerMenuApp {
    fn new() -> Self {
        Self {
            hover_scales: [1.0; 4],
        }
    }
}

impl Default for PowerMenuApp {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for PowerMenuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle escape key to close
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Escape) {
                let _ = Command::new("killall").arg("powermenu").output();
                std::process::exit(0);
            }
        });

        // Dark background matching the image (very dark, almost black)
        let background_color = egui::Color32::from_rgb(18, 18, 24);

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(background_color))
            .show(ctx, |ui| {
                let available_rect = ui.available_rect_before_wrap();
                let center = available_rect.center();

                // Button configuration
                let base_size = 220.0; // Large buttons
                let spacing = 30.0;

                // Calculate grid position
                let grid_w = base_size * 2.0 + spacing;
                let grid_h = base_size * 2.0 + spacing;
                let start_pos = center - egui::Vec2::new(grid_w / 2.0, grid_h / 2.0);

                // Button definitions: (Icon, BgColor, IconColor, Action)
                let buttons = [
                    // Top Left: Shutdown (Dark Red/Brown) - Using Nerd Font icon for better compatibility
                    (
                        "\u{f011}",
                        egui::Color32::from_rgb(85, 55, 60),
                        egui::Color32::from_rgb(255, 230, 230),
                        0,
                    ),
                    // Top Right: Lock (Light Lavender)
                    (
                        "🔒",
                        egui::Color32::from_rgb(210, 190, 255),
                        egui::Color32::from_rgb(40, 30, 50),
                        1,
                    ),
                    // Bottom Left: Suspend (Light Lavender)
                    (
                        "🌙",
                        egui::Color32::from_rgb(210, 190, 255),
                        egui::Color32::from_rgb(40, 30, 50),
                        2,
                    ),
                    // Bottom Right: Reboot (Dark Grey/Purple)
                    (
                        "↻",
                        egui::Color32::from_rgb(70, 65, 80),
                        egui::Color32::from_rgb(240, 240, 250),
                        3,
                    ),
                ];

                for (i, (icon, bg_color, icon_color, action_idx)) in buttons.iter().enumerate() {
                    let row = (i / 2) as f32;
                    let col = (i % 2) as f32;

                    let pos = start_pos
                        + egui::Vec2::new(col * (base_size + spacing), row * (base_size + spacing));

                    let rect = egui::Rect::from_min_size(pos, egui::Vec2::splat(base_size));

                    // Interaction
                    let response = ui.allocate_rect(rect, egui::Sense::click());
                    let is_hovered = response.hovered();

                    // Animation logic
                    let target_scale = if is_hovered { 1.05 } else { 1.0 };
                    let scale_speed = 10.0 * ctx.input(|i| i.stable_dt);
                    self.hover_scales[i] += (target_scale - self.hover_scales[i]) * scale_speed;

                    // Draw
                    let current_scale = self.hover_scales[i];
                    let center = rect.center();
                    let radius = (base_size / 2.0) * current_scale;

                    // Draw circle
                    ui.painter().circle_filled(center, radius, *bg_color);

                    // Draw icon
                    if *action_idx == 0 {
                        // Custom draw for Shutdown icon to avoid font issues
                        let icon_size = 64.0 * current_scale;
                        let stroke = egui::Stroke::new(icon_size * 0.08, *icon_color);
                        let radius = icon_size * 0.35;
                        let center_offset = egui::Vec2::new(0.0, icon_size * 0.05);
                        let icon_center = center + center_offset;

                        // Draw the arc
                        use std::f32::consts::PI;
                        let start_angle = -PI / 2.0 + 0.7; // Start after top gap
                        let end_angle = 3.0 * PI / 2.0 - 0.7; // End before top gap

                        let mut points = vec![];
                        let steps = 30;
                        for s in 0..=steps {
                            let t = s as f32 / steps as f32;
                            let angle = start_angle + t * (end_angle - start_angle);
                            points.push(
                                icon_center + egui::Vec2::new(angle.cos(), angle.sin()) * radius,
                            );
                        }
                        ui.painter().add(egui::Shape::line(points, stroke));

                        // Draw the vertical line
                        let line_top = icon_center + egui::Vec2::new(0.0, -radius * 1.1);
                        let line_bottom = icon_center + egui::Vec2::new(0.0, 0.0); // To center
                        ui.painter().line_segment([line_top, line_bottom], stroke);
                    } else {
                        let font_id = egui::FontId::proportional(64.0 * current_scale);
                        ui.painter().text(
                            center,
                            egui::Align2::CENTER_CENTER,
                            *icon,
                            font_id,
                            *icon_color,
                        );
                    }

                    // Handle clicks
                    if response.clicked() {
                        match action_idx {
                            0 => logic::shutdown(),
                            1 => logic::lockscreen(),
                            2 => {
                                logic::suspend();
                                logic::lockscreen();
                            }
                            3 => logic::reboot(),
                            _ => {}
                        }
                    }
                }
            });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_fullscreen(false)
            .with_decorations(false)
            .with_transparent(false)
            .with_always_on_top()
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "Power Menu",
        options,
        Box::new(move |cc| {
            setup_local_theme(None);
            if let Ok(mut theme) = get_global_theme().lock() {
                theme.theme_mode = ThemeMode::Dark;
            }
            load_fonts(cc.egui_ctx.clone());
            load_themes();
            update_window_background(cc.egui_ctx.clone());

            Ok(Box::new(PowerMenuApp::new()))
        }),
    )
}

