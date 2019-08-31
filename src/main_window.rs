use gdk::beep;
use gtk::prelude::*;
//use std::collections::HashMap;
use std::ops::Range;

use crate::state::State;

/// This represents the UI widgets for the window.
pub struct MainWindow {
	window: gtk::Window,
	buttons: Vec<gtk::Button>,
}

/// The number of channels this calc can handle.
/// Note that channels are 1-indexed, so ranges using this need to be `1..num_channels + 1`
pub const NUM_CHANNELS: i8 = 12;

impl MainWindow {
	pub fn new() -> MainWindow {
		// Initialize the UI from the Glade XML.
		let glade_src = include_str!("main_window.glade");
		let builder = gtk::Builder::new_from_string(glade_src);

		// Get handles for the various controls we need to use.
		let window: gtk::Window = builder.get_object("mainWindow").expect(&format!(
			"Could not get object 'mainWindow' from .glade file."
		));

		// add the buttons for each channel
		// Remember that Array's can't be initialised empty and then populated, so we'll make a Vec and then clone, rather than deal with Option<Button> till the end of days.
		let mut buttons: Vec<gtk::Button> = Vec::new();
		for channel in MainWindow::channel_range() {
			let name = format!("btn{}", channel);
			buttons.push(
				builder
					.get_object(&name)
					.expect(&format!("Could not get button '{}' from .glade file", name)),
			);
		}

		buttons.shrink_to_fit(); // ensure we only use as much memory as required.  This incurs a dealloc cost now, but reduced memory ongoing.
		MainWindow { window, buttons }
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
	fn update_channel(&self, channel: i8, value: i32, percent: f32) {
		let btn = self.get_button(channel);
		let text = format!(
			"Channel {}\nCount: {} ({:.2}%)",
			channel + 1, //account for 0-based indexing
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
				*i,
				c.get_count(),
				State::calc_percentage(c.get_count(), total),
			);
		}

		if total % state.alarm_count == 0 {
			// trigger a small alarm
			let num_beeps = total / state.alarm_count;
			for _ in 0..num_beeps {
				//NOTE: Multiple beeps not working atm... Needs ?delay?
				beep();
			}
		}
	}

	/// Helper function to get a range that covers all the channels correctly.
	pub fn channel_range() -> Range<i8> {
		0..NUM_CHANNELS
	}

	/// Gets the button associated with the given channel.  
	/// If passed an invalid channel this method will panic.
	pub fn get_button(&self, num: i8) -> &gtk::Button {
		&self.buttons[num as usize] //.expect(&format!("Could not get button {}.", num))
	}
}
