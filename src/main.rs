//use gtk::prelude::*;
use std::sync::Arc;

mod main_window;
use main_window::MainWindow;

fn main() {
    // Start up the GTK3 subsystem.
    gtk::init().expect("Unable to start GTK3. Error");
    // Create the main window.
    let gui = Arc::new(MainWindow::new());

    gui.start();
    gtk::main();
}
