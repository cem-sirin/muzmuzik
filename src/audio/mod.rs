pub mod buffer;
pub mod stream;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use buffer::push_sample;
use crate::instruments::SineWave;
use crate::note::Note;

pub use buffer::AudioBuffer;
pub use stream::get_default_output_device;

#[derive(Clone, Default)]
pub struct NoteState {
    pub playing: bool,
    pub phase: f32,
    pub start_time: Option<Instant>,
}

pub type AudioState = Arc<Mutex<HashMap<Note, NoteState>>>;
pub type OctaveOffset = Arc<Mutex<i32>>;

pub struct AudioEngine {
    pub state: AudioState,
    pub buffer: AudioBuffer,
    pub octave_offset: OctaveOffset,
    pub sample_rate: f32,
    pub buffer_capacity: usize,
}

impl AudioEngine {
    pub fn new(sample_rate: u32, buffer_ms: usize) -> Self {
        let buffer_capacity = (sample_rate as usize * buffer_ms) / 1000;
        Self {
            state: Arc::new(Mutex::new(HashMap::new())),
            buffer: buffer::new(buffer_capacity),
            octave_offset: Arc::new(Mutex::new(0)),
            sample_rate: sample_rate as f32,
            buffer_capacity,
        }
    }

    pub fn build_audio_callback(&self) -> AudioCallback {
        let state = Arc::clone(&self.state);
        let buffer = Arc::clone(&self.buffer);
        let sample_rate = self.sample_rate;
        let buffer_capacity = self.buffer_capacity;

        AudioCallback {
            state,
            buffer,
            sample_rate,
            buffer_capacity,
        }
    }

    pub fn note_on(&self, note: Note) {
        let mut states = self.state.lock().unwrap();
        let entry = states.entry(note).or_default();
        if !entry.playing {
            entry.playing = true;
            entry.phase = 0.0;
            entry.start_time = Some(Instant::now());
        }
    }

    pub fn note_off(&self, note: Note) {
        let mut states = self.state.lock().unwrap();
        if let Some(entry) = states.get_mut(&note) {
            entry.playing = false;
        }
    }

    pub fn change_octave(&self, delta: i32) {
        let mut octave = self.octave_offset.lock().unwrap();
        *octave += delta;
    }

    pub fn get_octave(&self) -> i32 {
        *self.octave_offset.lock().unwrap()
    }

    pub fn get_playing_notes(&self) -> Vec<Note> {
        let states = self.state.lock().unwrap();
        states
            .iter()
            .filter(|(_, s)| s.playing)
            .map(|(n, _)| *n)
            .collect()
    }
}

pub struct AudioCallback {
    state: AudioState,
    buffer: AudioBuffer,
    sample_rate: f32,
    buffer_capacity: usize,
}

impl AudioCallback {
    pub fn into_callback(self) -> impl FnMut(&mut [f32], &cpal::OutputCallbackInfo) + Send + 'static {
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut states = self.state.lock().unwrap();

            for sample in data.iter_mut() {
                let mut total_sample = 0.0;
                for (note, note_state) in states.iter_mut() {
                    if note_state.playing {
                        let freq = note.frequency_hz();
                        total_sample += SineWave::sample(note_state.phase) * 0.1;
                        note_state.phase += freq / self.sample_rate;
                        if note_state.phase > 1.0 {
                            note_state.phase -= 1.0;
                        }
                    }
                }
                *sample = total_sample;
                push_sample(&self.buffer, total_sample, self.buffer_capacity);
            }
        }
    }
}
