use std::cell::RefCell;
use std::collections::HashMap;

use crate::channel::Channel;

/// Reflects the current running state of the application.
pub struct State {
	pub channels: RefCell<HashMap<i8, Channel>>,
	/// Trigger a small alarm every `alarm_count` total entries, so users know how many they're up to.
	pub alarm_count: i32,
}

impl State {
	/// Gets the current total count of all channels.
	/// This needs to borrow the channels, so can't be called if there's an outstanding mut borrow.
	pub fn total_count(&self) -> i32 {
		self.channels
			.borrow()
			.iter()
			.map(|(_, c)| c.get_count())
			.sum()
	}

	/// Simple helper function to calc a percentage.
	pub fn calc_percentage(x: i32, t: i32) -> f32 {
		if t == 0 {
			return 0.0;
		}

		return x as f32 / t as f32;
	}
}
