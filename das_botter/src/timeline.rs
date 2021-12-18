use crate::send_input::{self};
use crate::{key::Key, toggle::Toggle};
use std::time::Duration;

const CAPACITY: usize = 10000000;

pub struct Timeline {
    queue: Vec<(Duration, Key, Toggle)>,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            queue: Vec::with_capacity(CAPACITY),
        }
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }

    pub fn queue(&mut self, duration: Duration, key: Key, toggle: Toggle) {
        self.queue.push((duration, key, toggle));
    }

    pub fn execute(&self) {
        for (duration, key, toggle) in self.queue.iter() {
            std::thread::sleep(*duration);
            send_input::send_keyboard(*key, *toggle).unwrap();
        }
    }
}
