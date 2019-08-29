pub struct Channel {
	channel_num: i8,
	count: i32,
}

impl Channel {
	pub fn new(channel_num: i8) -> Channel {
		Channel {
			channel_num,
			count: 0,
		}
	}

	pub fn get_count(&self) -> i32 {
		self.count
	}

	pub fn increment(&mut self) -> () {
		self.count += 1;
	}
}
