use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box, Button, CssProvider, Label, StyleContext};
use gtk4 as gtk;
use std::process::Command;
mod logic;

fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("com.example.TwoButtonsApp")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .fullscreened(true) // Open in fullscreen mode
            .decorated(false) // Remove window decorations
            .build();

        window.set_opacity(0.8); // Adjust transparency level

        // CSS styling for buttons and blur effect
        let css = r#"
            window {
                background: rgba(0, 0, 0, 0.6);
                backdrop-filter: blur(20px); /* Apply blur effect */
            }

            button {
                background-color: #908caa; /* Initial solid button background */
                color: #000000; /* Text color */
                border: 2px solid #c4a7e7; /* Border color */
                border-radius: 8px; /* Rounded corners */
                width: 100px;
                height: 100px; /* Make buttons square */
                margin: 10px; /* Space between buttons */
                font-size: 14px;
                transition: background-color 0.3s ease, transform 0.2s ease; /* Smooth transition */
            }

            /* Hover effect for buttons */
            button:hover {
                background-color: #c4a7e7; /* Change background on hover */
                transform: scale(1.1); /* Slightly increase size */
            }

            box {
                justify-content: center; /* Center buttons horizontally */
                align-items: center; /* Center buttons vertically */
            }
        "#;

        let provider = CssProvider::new();
        provider.load_from_data(css);
        StyleContext::add_provider_for_display(
            &gtk::gdk::Display::default().expect("Could not get display"),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // Create a vertical Box to center the buttons
        let vbox = Box::new(gtk::Orientation::Vertical, 10);
        vbox.set_halign(gtk::Align::Center);
        vbox.set_valign(gtk::Align::Center);

        // Create buttons with specific labels
        let button1 = Button::with_label("Shutdown");
        button1.connect_clicked(|_| {
            logic::shutdown();
        });

        let button2 = Button::with_label("Reboot");
        button2.connect_clicked(|_| {
            logic::reboot();
        });
        let button3 = Button::with_label("Suspend");
        button3.connect_clicked(|_| {
            logic::suspend();
            logic::lockscreen();
        });

        let button4 = Button::with_label("Logout");
        button4.connect_clicked(|_| {
            logic::logout();
        });

        // Add buttons to the Box
        vbox.append(&button1);
        vbox.append(&button2);
        vbox.append(&button3);
        vbox.append(&button4);
        let label = Label::new(Some("Press SUPER+C to close the program"));
        vbox.append(&label); // Add label below buttons
                             // Set the Box as the child of the window
        window.set_child(Some(&vbox));

        window.present();
    });

    application.run()
}
