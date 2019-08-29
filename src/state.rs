use std::cell::RefCell;
use std::collections::HashMap;

use crate::channel::Channel;

pub struct State {
	pub channels: RefCell<HashMap<i8, Channel>>,
}

impl State {
	pub fn total_count(&self) -> i32 {
		self.channels
			.borrow() //this crashes because the btn handler has a mutable borrow in order to increment the channel
			.iter()
			.map(|(_, c)| c.get_count())
			.sum()
	}
}
