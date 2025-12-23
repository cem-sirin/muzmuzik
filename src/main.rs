use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
        KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Paragraph},
};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::Instant;

mod note;
mod widgets;

use note::{Note, NoteLetter};

const KEYS: &str = "qwertyuiop[]asdfghjkl{},";
const LETTERS: [NoteLetter; 12] = [
    NoteLetter::A,
    NoteLetter::B,
    NoteLetter::C,
    NoteLetter::D,
    NoteLetter::E,
    NoteLetter::F,
    NoteLetter::G,
    NoteLetter::H,
    NoteLetter::I,
    NoteLetter::J,
    NoteLetter::K,
    NoteLetter::L,
];
const WAVEFORM_BUFFER_MS: usize = 200;

#[derive(Clone)]
struct NoteState {
    playing: bool,
    phase: f32,
    start_time: Option<Instant>,
}

type AudioState = Arc<Mutex<HashMap<char, NoteState>>>;
type AudioBuffer = Arc<Mutex<VecDeque<f32>>>;
type OctaveOffset = Arc<Mutex<i32>>;

fn idx_to_note(idx: usize, octave_offset: i32) -> Note {
    let base_octave = 1 + (idx / 12) as i32;
    let letter = LETTERS[idx % 12];
    Note::new(letter, base_octave + octave_offset)
}

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
    let supported_config = device.default_output_config().expect("no supported config");
    let config = supported_config.config();
    let sample_rate_hz = config.sample_rate as f32;

    let audio_state: AudioState = Arc::new(Mutex::new(HashMap::new()));
    let audio_state_clone = Arc::clone(&audio_state);
    let waveform_buffer_size = ((config.sample_rate as usize) * WAVEFORM_BUFFER_MS) / 1000;

    let audio_buffer: AudioBuffer =
        Arc::new(Mutex::new(VecDeque::with_capacity(waveform_buffer_size)));
    let audio_buffer_clone = Arc::clone(&audio_buffer);

    let octave_offset: OctaveOffset = Arc::new(Mutex::new(0));
    let octave_offset_clone = Arc::clone(&octave_offset);

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let octave = *octave_offset_clone.lock().unwrap();

            let mut states = audio_state_clone.lock().unwrap();
            let mut buffer = audio_buffer_clone.lock().unwrap();
            for sample in data.iter_mut() {
                let mut total_sample = 0.0;
                for (key, state) in states.iter_mut() {
                    if state.playing {
                        if let Some(idx) = KEYS.find(*key) {
                            let note = idx_to_note(idx, octave);
                            let freq = note.frequency_hz();
                            total_sample += (state.phase * 2.0 * std::f32::consts::PI).sin() * 0.1;
                            state.phase += freq / sample_rate_hz;
                            if state.phase > 1.0 {
                                state.phase -= 1.0;
                            }
                        }
                    }
                }
                *sample = total_sample;
                buffer.push_back(total_sample);
                if buffer.len() > waveform_buffer_size {
                    buffer.pop_front();
                }
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
    execute!(
        stdout,
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        // Draw TUI
        let playing_chars: std::collections::HashSet<char> = {
            let states = audio_state.lock().unwrap();
            states
                .iter()
                .filter(|(_, s)| s.playing)
                .map(|(k, _)| *k)
                .collect()
        };
        terminal.draw(|f| {
            let octave = *octave_offset.lock().unwrap();
            let size = f.area();
            let bg_block = Block::default().style(Style::default().bg(Color::Black));
            f.render_widget(bg_block, size);
            // Split vertically: top charts, empty space, keys, shortcuts banner.
            let vertical_areas = Layout::vertical([
                Constraint::Ratio(1, 2),
                Constraint::Min(0),
                Constraint::Ratio(1, 7),
                Constraint::Length(1),
            ])
            .split(size);
            // Waveform + spectrum at top
            let top_area = vertical_areas[0];
            let top_areas = Layout::vertical([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
                .split(top_area);

            let waveform_widget = widgets::Waveform {
                buffer: &audio_buffer,
            };
            f.render_widget(waveform_widget, top_areas[0]);

            let spectrum_widget = widgets::Spectrum {
                buffer: &audio_buffer,
                sample_rate: config.sample_rate,
            };
            f.render_widget(spectrum_widget, top_areas[1]);
            // Keys in third area (centered)
            let key_area = vertical_areas[2];
            // Guarantee identical heights: if odd, reserve 1 line as a spacer.
            let reserved = key_area.height % 2;
            let usable_height = key_area.height.saturating_sub(reserved);
            let row_height = usable_height / 2;

            let key_rows: [Rect; 2] = [
                Rect {
                    x: key_area.x,
                    y: key_area.y,
                    width: key_area.width,
                    height: row_height,
                },
                Rect {
                    x: key_area.x,
                    y: key_area.y + row_height + reserved,
                    width: key_area.width,
                    height: row_height,
                },
            ];
            // First row: 12 keys
            let areas1 = Layout::horizontal(vec![Constraint::Ratio(1, 12); 12]).split(key_rows[0]);
            for i in 0..12 {
                let key = KEYS.as_bytes()[i] as char;
                let note = idx_to_note(i, octave);
                let pressed = playing_chars.contains(&key);
                let color = widgets::note_color(i % 12);
                let key_widget = widgets::PianoKey {
                    label: note.to_string(),
                    color,
                    pressed,
                };
                f.render_widget(key_widget, areas1[i]);
            }
            // Second row: 12 keys
            let areas2 = Layout::horizontal(vec![Constraint::Ratio(1, 12); 12]).split(key_rows[1]);
            for i in 12..24 {
                let idx = i - 12;
                let key = KEYS.as_bytes()[i] as char;
                let note = idx_to_note(i, octave);
                let pressed = playing_chars.contains(&key);
                let color = widgets::note_color(idx % 12);
                let key_widget = widgets::PianoKey {
                    label: note.to_string(),
                    color,
                    pressed,
                };
                f.render_widget(key_widget, areas2[idx]);
            }

            // Bottom shortcuts banner
            let banner_area = vertical_areas[3];
            let banner = format!(
                "shortcuts: n = -1 octave | m = +1 octave | ctrl-c = quit | offset: {}",
                octave
            );
            let banner_widget = Paragraph::new(banner).style(Style::default().fg(Color::DarkGray));
            f.render_widget(banner_widget, banner_area);
        })?;

        // Handle events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.kind {
                    KeyEventKind::Press => {
                        if key.code == KeyCode::Char('c')
                            && key.modifiers.contains(KeyModifiers::CONTROL)
                        {
                            break;
                        }
                        if let KeyCode::Char(c) = key.code {
                            if c == 'n' {
                                let mut octave = octave_offset.lock().unwrap();
                                *octave -= 1;
                            } else if c == 'm' {
                                let mut octave = octave_offset.lock().unwrap();
                                *octave += 1;
                            } else if KEYS.contains(c) {
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
