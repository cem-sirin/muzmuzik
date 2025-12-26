use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub type AudioBuffer = Arc<Mutex<VecDeque<f32>>>;

pub fn new(buffer_size: usize) -> AudioBuffer {
    Arc::new(Mutex::new(VecDeque::with_capacity(buffer_size)))
}

pub fn push_sample(buffer: &AudioBuffer, sample: f32, capacity: usize) {
    let mut buf = buffer.lock().unwrap();
    buf.push_back(sample);
    if buf.len() > capacity {
        buf.pop_front();
    }
}
