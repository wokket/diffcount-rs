/// Reflects the state and management of a single channel on the counter.
#[allow(dead_code)]
pub struct Channel {
	pub channel_num: i8, //this feels like important info, yet we haven't used it yet...
	count: i32,
}

#[warn(dead_code)]
impl Channel {
	pub fn new(channel_num: i8) -> Channel {
		Channel {
			channel_num,
			count: 0,
		}
	}

	/// Get the current count of this channel.  This is guaranteed to be >= 0.
	pub fn get_count(&self) -> i32 {
		self.count
	}

	/// Increments the counter on this channel.
	pub fn increment(&mut self) -> () {
		self.count += 1;
	}
}
