use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers, KeyboardEnhancementFlags, PushKeyboardEnhancementFlags, PopKeyboardEnhancementFlags},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn note_freq(n: usize) -> f32 {
    BASE_FREQ * (2.0_f32).powf(n as f32 / 12.0)
}

const KEYS: &str = "qwertyuiop[]";
const LABELS: &str = "ABCDEFGHIJKL";
const SAMPLE_RATE: u32 = 44100;
const BASE_FREQ: f32 = 261.63; // A1 in your notation, maps to C3

#[derive(Clone)]
struct NoteState {
    playing: bool,
    phase: f32,
    start_time: Option<Instant>,
}

type AudioState = Arc<Mutex<HashMap<char, NoteState>>>;

impl Default for NoteState {
    fn default() -> Self {
        Self {
            playing: false,
            phase: 0.0,
            start_time: None,
        }
    }
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup audio
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");
    let supported_config = device
        .default_output_config()
        .expect("no supported config");
    let config = supported_config.config();

    let audio_state: AudioState = Arc::new(Mutex::new(HashMap::new()));
    let audio_state_clone = Arc::clone(&audio_state);

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut states = audio_state_clone.lock().unwrap();
            for sample in data.iter_mut() {
                let mut total_sample = 0.0;
                for (key, state) in states.iter_mut() {
                    if state.playing {
                        if let Some(idx) = KEYS.find(*key) {
                            let freq = note_freq(idx);
                            total_sample += (state.phase * 2.0 * std::f32::consts::PI).sin() * 0.1;
                            state.phase += freq / SAMPLE_RATE as f32;
                            if state.phase > 2.0 * std::f32::consts::PI {
                                state.phase -= 2.0 * std::f32::consts::PI;
                            }
                        }
                    }
                }
                *sample = total_sample;
            }
        },
        |err| eprintln!("audio error: {}", err),
        None,
    )?;

    stream.play()?;

    // Setup TUI
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    // Enable Kitty Keyboard Protocol for reliable key release events
    execute!(stdout, PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES))?; 
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;



    loop {
        // Draw TUI
        let playing_chars: std::collections::HashSet<char> = {
            let states = audio_state.lock().unwrap();
            states.iter().filter(|(_, s)| s.playing).map(|(k, _)| *k).collect()
        };
        terminal.draw(|f| {
            let size = f.area();
            let mut text = String::new();
            for (key, label) in KEYS.chars().zip(LABELS.chars()) {
                if playing_chars.contains(&key) {
                    text.push_str(&format!("{}:*{}* ", key, label));
                } else {
                    text.push_str(&format!("{}:{} ", key, label));
                }
            }
            let paragraph = Paragraph::new(text)
                .block(Block::default().title("Piano").borders(Borders::ALL));
            f.render_widget(paragraph, size);
        })?;

        // Handle events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.kind {
                    KeyEventKind::Press => {
                        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                            break;
                        }
                        if let KeyCode::Char(c) = key.code {
                            if KEYS.contains(c) {
                                let mut states = audio_state.lock().unwrap();
                                let entry = states.entry(c).or_insert(NoteState::default());
                                if !entry.playing {
                                    entry.playing = true;
                                    entry.phase = 0.0;
                                    entry.start_time = Some(Instant::now());
                                }
                            }
                        }
                    }
                    KeyEventKind::Release => {
                        if let KeyCode::Char(c) = key.code {
                            if KEYS.contains(c) {
                                let mut states = audio_state.lock().unwrap();
                                if let Some(entry) = states.get_mut(&c) {
                                    entry.playing = false;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // Cleanup
    execute!(terminal.backend_mut(), PopKeyboardEnhancementFlags)?;
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
