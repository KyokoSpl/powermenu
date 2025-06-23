use std::process::Command;

use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box, Button, CssProvider, Label};
use gtk4 as gtk;
use gtk4::glib::Propagation;
mod logic;

fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("com.example.TwoButtonsApp")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .fullscreened(true) // Open in fullscreen mode
            .decorated(true) // Remove window decorations
            .default_width(750)
            .default_height(250)
            .build();

        window.set_opacity(1.0); // Adjust transparency level

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
        gtk::style_context_add_provider_for_display(
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
        let button5 = Button::with_label("Lockscreen");
        button5.connect_clicked(|_| {
            logic::lockscreen();
        });

        // Add buttons to the Box
        vbox.append(&button1);
        vbox.append(&button2);
        vbox.append(&button3);
        vbox.append(&button4);
        vbox.append(&button5);

        let label = Label::new(Some("Press ESC to close the program"));
        vbox.append(&label);
        window.set_child(Some(&vbox));

        let key_controller = gtk::EventControllerKey::new();
        key_controller.connect_key_pressed(move |_, key, _, _| {
            if key == gtk::gdk::Key::Escape {
                println!("Escape key pressed, closing the application");
                let _ = Command::new("killall").arg("powermenu").output();
                Propagation::Stop // Stop further propagation
            } else {
                println!("Key pressed: {:?}", key);
                Propagation::Proceed // Continue propagating the event
            }
        });
        window.add_controller(key_controller);

        window.present();
    });

    application.run()
}
