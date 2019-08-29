use gdk::beep;
use gtk::prelude::*;
use std::collections::HashMap;
use std::ops::Range;

use crate::state::State;

/// This represents the UI widgets for the window.
pub struct MainWindow {
	window: gtk::Window,
	buttons: HashMap<i8, gtk::Button>,

	/// The number of channels this calc can handle.
	/// Note that channels are 1-indexed, so ranges using this need to be `1..num_channels + 1`
	num_channels: i8,
}

impl MainWindow {
	pub fn new() -> MainWindow {
		let num_channels = 12;

		// Initialize the UI from the Glade XML.
		let glade_src = include_str!("main_window.glade");
		let builder = gtk::Builder::new_from_string(glade_src);

		// Get handles for the various controls we need to use.
		let window: gtk::Window = builder.get_object("mainWindow").expect(&format!(
			"Could not get object 'mainWindow' from .glade file."
		));

		//add the buttons for each channel
		let mut buttons: HashMap<i8, gtk::Button> = HashMap::new();
		for channel in MainWindow::channel_range_internal(num_channels) {
			let name = format!("btn{}", channel);
			buttons.insert(
				channel,
				builder
					.get_object(&name)
					.expect(&format!("Could not get button '{}' from .glade file", name)),
			);
		}

		MainWindow {
			window,
			buttons,
			num_channels,
		}
	}

	/// Set up naming for the window and show it to the user.
	pub fn start(&self) {
		glib::set_application_name("Diff Counter");
		self.window.connect_delete_event(|_, _| {
			gtk::main_quit();
			Inhibit(false)
		});
		self.window.resize(640, 400);
		self.window.show_all();
	}

	/// Updates a single channel of data, generally dangerous to call in
	/// isolation as percentages generally require a global update.
	fn update_channel(&self, channel: &i8, value: i32, percent: f32) {
		let btn = self.get_button(channel);
		let text = format!(
			"Channel {}\nCount: {} ({:.2}%)",
			channel,
			value,
			&percent * 100.0
		);
		btn.set_label(&text);
	}

	/// Triggers a full update of the UI to reflect the nature of the given `State`
	pub fn update(&self, state: &State) {
		let total = state.total_count();

		self.window
			.set_title(&format!("Diff Counter (Total: {})", total));

		for (i, c) in state.channels.borrow().iter() {
			self.update_channel(
				i,
				c.get_count(),
				State::calc_percentage(c.get_count(), total),
			);
		}

		if total % state.alarm_count == 0 {
			// trigger a small alarm
			beep();
		}
	}

	/// Helper function to get a range that covers all the channels correctly.
	pub fn channel_range(&self) -> Range<i8> {
		MainWindow::channel_range_internal(self.num_channels)
	}

	fn channel_range_internal(i: i8) -> Range<i8> {
		1..i + 1
	}

	/// Gets the button associated with the given channel.  
	/// If passed an invalid channel this method will panic.
	pub fn get_button(&self, num: &i8) -> &gtk::Button {
		self.buttons
			.get(num)
			.expect(&format!("Could not get button {}.", num))
	}
}
