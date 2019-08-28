use gtk::prelude::*;
//use std::collections::HashMap;

pub struct MainWindow {
	window: gtk::Window,
	// result: gtk::Label,
	// popover: gtk::Popover,
	// error_label: gtk::Label,
	// user_spec_entry: gtk::Entry,
	// buttons: HashMap<String, gtk::Button>,
}

impl MainWindow {
	pub fn new() -> MainWindow {
		// Initialize the UI from the Glade XML.
		let glade_src = include_str!("main_window.glade");
		let builder = gtk::Builder::new_from_string(glade_src);

		// Get handles for the various controls we need to use.
		let window: gtk::Window = builder.get_object("mainWindow").unwrap();

		MainWindow {
			window,
			// result,
			// popover,
			// error_label,
			// user_spec_entry,
			// buttons,
		}
	}

	// Set up naming for the window and show it to the user.
	pub fn start(&self) {
		glib::set_application_name("diffCounter");
		self.window.set_wmclass("Diff Counter", "Diff Counter");
		self.window.connect_delete_event(|_, _| {
			gtk::main_quit();
			Inhibit(false)
		});
		self.window.show_all();
	}
}
