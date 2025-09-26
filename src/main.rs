use std::process::Command;
use std::path::Path;

use eframe::egui;
use egui_material3::{
    MaterialButton,
    theme::{setup_local_theme, load_fonts, load_themes, update_window_background, ThemeMode, get_global_theme}
};

mod logic;

#[derive(Clone)]
struct ThemeColors {
    background: egui::Color32,
    button_hover: egui::Color32,
    title_text: egui::Color32,
    subtitle_text: egui::Color32,
}

impl Default for ThemeColors {
    fn default() -> Self {
        // Default purple theme
        Self {
            background: egui::Color32::from_rgba_unmultiplied(30, 30, 46, 240),
            button_hover: egui::Color32::from_rgb(196, 167, 231),
            title_text: egui::Color32::WHITE,
            subtitle_text: egui::Color32::LIGHT_GRAY,
        }
    }
}

fn detect_theme_colors() -> ThemeColors {
    // Check for existing theme files and extract colors
    let style_dir = "styles";
    
    // Try to detect which theme to use based on available CSS files
    // Prioritizing purple/violet themes first
    let theme_files = vec![
        ("catppuccin.css", ThemeColors {
            background: egui::Color32::from_rgba_unmultiplied(17, 17, 27, 250), // Darker Catppuccin base
            button_hover: egui::Color32::from_rgb(203, 166, 247), // Catppuccin mauve - beautiful purple
            title_text: egui::Color32::from_rgb(245, 224, 220), // Catppuccin rosewater
            subtitle_text: egui::Color32::from_rgb(186, 194, 222), // Catppuccin subtext0
        }),
        ("dracula.css", ThemeColors {
            background: egui::Color32::from_rgba_unmultiplied(40, 42, 54, 250), // #282a36
            button_hover: egui::Color32::from_rgb(189, 147, 249), // Dracula purple - classic
            title_text: egui::Color32::from_rgb(255, 121, 198), // Dracula pink
            subtitle_text: egui::Color32::from_rgb(139, 233, 253), // Dracula cyan
        }),
        ("onedark.css", ThemeColors {
            background: egui::Color32::from_rgba_unmultiplied(40, 44, 52, 250),
            button_hover: egui::Color32::from_rgb(198, 120, 221), // OneDark purple
            title_text: egui::Color32::from_rgb(224, 108, 117), // OneDark red
            subtitle_text: egui::Color32::from_rgb(152, 195, 121), // OneDark green
        }),
        ("nord.css", ThemeColors {
            background: egui::Color32::from_rgba_unmultiplied(46, 52, 64, 250), // Nord polar night
            button_hover: egui::Color32::from_rgb(143, 188, 187), // Nord frost - teal purple
            title_text: egui::Color32::from_rgb(136, 192, 208), // Nord frost
            subtitle_text: egui::Color32::from_rgb(216, 222, 233), // Nord snow storm
        }),
        ("gruv.css", ThemeColors {
            background: egui::Color32::from_rgba_unmultiplied(40, 40, 40, 250), // Gruvbox dark
            button_hover: egui::Color32::from_rgb(211, 134, 155), // Gruvbox purple
            title_text: egui::Color32::from_rgb(254, 128, 25), // Gruvbox orange
            subtitle_text: egui::Color32::from_rgb(184, 187, 38), // Gruvbox green
        }),
        ("solarized.css", ThemeColors {
            background: egui::Color32::from_rgba_unmultiplied(0, 43, 54, 250), // Solarized dark
            button_hover: egui::Color32::from_rgb(211, 54, 130), // Solarized magenta - purple-ish
            title_text: egui::Color32::from_rgb(42, 161, 152), // Solarized cyan
            subtitle_text: egui::Color32::from_rgb(147, 161, 161), // Solarized base1
        }),
    ];
    
    // Check which theme file exists and use it, prioritizing purple themes
    for (filename, colors) in theme_files {
        if Path::new(&format!("{}/{}", style_dir, filename)).exists() {
            println!("Using theme colors from: {}", filename);
            return colors;
        }
    }
    
    // Fallback to a beautiful default purple theme
    println!("Using default purple theme");
    ThemeColors {
        background: egui::Color32::from_rgba_unmultiplied(25, 25, 35, 250), // Deep dark purple
        button_hover: egui::Color32::from_rgb(147, 112, 219), // Medium slate blue
        title_text: egui::Color32::from_rgb(221, 160, 221), // Plum
        subtitle_text: egui::Color32::from_rgb(186, 85, 211), // Medium orchid
    }
}

fn main() -> Result<(), eframe::Error> {
    let theme_colors = detect_theme_colors();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_fullscreen(true)  // Launch in fullscreen
            .with_decorations(false)
            .with_transparent(false)  // Disable transparency for fullscreen
            .with_always_on_top()
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "Power Menu",
        options,
        Box::new(move |cc| {
            // Setup Material Design fonts and themes
            setup_local_theme(None); // Use default theme
            
            // Set dark mode
            if let Ok(mut theme) = get_global_theme().lock() {
                theme.theme_mode = ThemeMode::Dark;
            }
            
            // Load fonts and themes
            load_fonts(cc.egui_ctx.clone());
            load_themes();
            
            // Apply theme background
            update_window_background(cc.egui_ctx.clone());
            
            Ok(Box::new(PowerMenuApp::new(theme_colors)))
        }),
    )
}

struct PowerMenuApp {
    theme_colors: ThemeColors,
    hover_states: [bool; 5], // Track hover state for each button
}

impl PowerMenuApp {
    fn new(theme_colors: ThemeColors) -> Self {
        Self { 
            theme_colors,
            hover_states: [false; 5],
        }
    }
}

impl Default for PowerMenuApp {
    fn default() -> Self {
        Self::new(ThemeColors::default())
    }
}

impl eframe::App for PowerMenuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle escape key to close
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Escape) {
                println!("Escape key pressed, closing the application");
                let _ = Command::new("killall").arg("powermenu").output();
                std::process::exit(0);
            }
        });

        // Fullscreen background with theme colors
        egui::CentralPanel::default()
            .frame(egui::Frame::new()
                .fill(self.theme_colors.background)
                .inner_margin(20.0))
            .show(ctx, |ui| {
                // Use available space efficiently with scrolling if needed
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            // Get available space and calculate layout
                            let available_height = ui.available_height().max(600.0); // Minimum height
                            let title_height = 80.0; // Space for title
                            let help_height = 40.0; // Space for help text
                            let button_count = 5.0;
                            let max_button_height = 55.0;
                            let min_spacing = 6.0;
                            
                            // Calculate optimal spacing and button height
                            let remaining_height = available_height - title_height - help_height;
                            let total_spacing = remaining_height - (button_count * max_button_height);
                            let spacing = (total_spacing / (button_count + 1.0)).max(min_spacing).min(16.0);
                    
                    ui.add_space(spacing * 0.5);
                    
                    // Title with theme colors
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new("Power Menu")
                            .size(36.0)
                            .color(self.theme_colors.title_text)
                            .strong());
                        ui.label(egui::RichText::new("Choose an action")
                            .size(16.0)
                            .color(self.theme_colors.subtitle_text));
                    });
                    
                    ui.add_space(spacing);

                    // Custom styled buttons with theme colors
                    let button_size = [350.0, max_button_height];

                            // Shutdown button
                            let shutdown_response = ui.add_sized(button_size, 
                                MaterialButton::filled(
                                    egui::RichText::new("⏻  Shutdown")
                                        .color(self.theme_colors.background)
                                )
                                .fill(self.theme_colors.button_hover)
                                .corner_radius(if self.hover_states[0] { 12.0 } else { button_size[1] / 2.0 })
                                .min_size(egui::Vec2::new(button_size[0], button_size[1]))
                            );
                            self.hover_states[0] = shutdown_response.hovered();
                            if shutdown_response.clicked() {
                                logic::shutdown();
                            }
                            
                            ui.add_space(spacing);

                            // Reboot button
                            let reboot_response = ui.add_sized(button_size, 
                                MaterialButton::filled(
                                    egui::RichText::new("↻  Reboot")
                                        .color(self.theme_colors.background)
                                )
                                .fill(self.theme_colors.button_hover)
                                .corner_radius(if self.hover_states[1] { 12.0 } else { button_size[1] / 2.0 })
                                .min_size(egui::Vec2::new(button_size[0], button_size[1]))
                            );
                            self.hover_states[1] = reboot_response.hovered();
                            if reboot_response.clicked() {
                                logic::reboot();
                            }
                            
                            ui.add_space(spacing);

                            // Suspend button
                            let suspend_response = ui.add_sized(button_size, 
                                MaterialButton::filled(
                                    egui::RichText::new("⏸  Suspend")
                                        .color(self.theme_colors.background)
                                )
                                .fill(self.theme_colors.button_hover)
                                .corner_radius(if self.hover_states[2] { 12.0 } else { button_size[1] / 2.0 })
                                .min_size(egui::Vec2::new(button_size[0], button_size[1]))
                            );
                            self.hover_states[2] = suspend_response.hovered();
                            if suspend_response.clicked() {
                                logic::suspend();
                                logic::lockscreen();
                            }
                            
                            ui.add_space(spacing);

                            // Logout button
                            let logout_response = ui.add_sized(button_size, 
                                MaterialButton::filled(
                                    egui::RichText::new("👤  Logout")
                                        .color(self.theme_colors.background)
                                )
                                .fill(self.theme_colors.button_hover)
                                .corner_radius(if self.hover_states[3] { 12.0 } else { button_size[1] / 2.0 })
                                .min_size(egui::Vec2::new(button_size[0], button_size[1]))
                            );
                            self.hover_states[3] = logout_response.hovered();
                            if logout_response.clicked() {
                                logic::logout();
                            }
                            
                            ui.add_space(spacing);

                            // Lockscreen button
                            let lockscreen_response = ui.add_sized(button_size, 
                                MaterialButton::filled(
                                    egui::RichText::new("🔒  Lock Screen")
                                        .color(self.theme_colors.background)
                                )
                                .fill(self.theme_colors.button_hover)
                                .corner_radius(if self.hover_states[4] { 12.0 } else { button_size[1] / 2.0 })
                                .min_size(egui::Vec2::new(button_size[0], button_size[1]))
                            );
                            self.hover_states[4] = lockscreen_response.hovered();
                            if lockscreen_response.clicked() {
                                logic::lockscreen();
                            }                    ui.add_space(spacing * 0.5);
                    
                            // Help text
                            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                ui.label(egui::RichText::new("Press ESC to close")
                                    .size(14.0)
                                    .color(self.theme_colors.subtitle_text));
                            });
                        });
                    });
            });
    }
}
