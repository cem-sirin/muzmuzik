pub mod event_handler;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyboardEnhancementFlags},
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
use std::collections::HashSet;

use crate::audio::AudioEngine;

pub fn run_tui(
    engine: &AudioEngine,
    sample_rate: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    execute!(
        stdout,
        crossterm::event::PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        let playing_notes: HashSet<_> = engine.get_playing_notes().into_iter().collect();
        let octave = engine.get_octave();

        terminal.draw(|f| {
            let size = f.area();
            let bg_block = Block::default().style(Style::default().bg(Color::Black));
            f.render_widget(bg_block, size);

            let vertical_areas = Layout::vertical([
                Constraint::Ratio(1, 2),
                Constraint::Min(0),
                Constraint::Ratio(1, 7),
                Constraint::Length(1),
            ])
            .split(size);

            let top_area = vertical_areas[0];
            let top_areas = Layout::vertical([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
                .split(top_area);

            let waveform_widget = crate::widgets::Waveform {
                buffer: &engine.buffer,
            };
            f.render_widget(waveform_widget, top_areas[0]);

            let spectrum_widget = crate::widgets::Spectrum {
                buffer: &engine.buffer,
                sample_rate,
            };
            f.render_widget(spectrum_widget, top_areas[1]);

            let key_area = vertical_areas[2];
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

            let areas1 = Layout::horizontal(vec![Constraint::Ratio(1, 12); 12]).split(key_rows[0]);
            for i in 0..12 {
                let key = crate::piano::get_key_at_index(i);
                let note = crate::piano::key_to_note(key, octave);
                let pressed = note.is_some_and(|n| playing_notes.contains(&n));
                let color = crate::widgets::note_color(i % 12);
                let key_widget = crate::widgets::PianoKey {
                    label: note.map(|n| n.to_string()).unwrap_or_else(|| "?".to_string()),
                    color,
                    pressed,
                };
                f.render_widget(key_widget, areas1[i]);
            }

            let areas2 = Layout::horizontal(vec![Constraint::Ratio(1, 12); 12]).split(key_rows[1]);
            for i in 12..24 {
                let key = crate::piano::get_key_at_index(i);
                let note = crate::piano::key_to_note(key, octave);
                let pressed = note.is_some_and(|n| playing_notes.contains(&n));
                let color = crate::widgets::note_color(i % 12);
                let key_widget = crate::widgets::PianoKey {
                    label: note.map(|n| n.to_string()).unwrap_or_else(|| "?".to_string()),
                    color,
                    pressed,
                };
                f.render_widget(key_widget, areas2[i - 12]);
            }

            let banner_area = vertical_areas[3];
            let keys_preview = crate::piano::get_mapping();
            let keys_preview = if keys_preview.len() > 12 {
                &keys_preview[..12]
            } else {
                keys_preview
            };
            let banner = format!(
                "keys: {}... | n/m: octave offset | ctrl-c: quit",
                keys_preview
            );
            let banner_widget = Paragraph::new(banner).style(Style::default().fg(Color::DarkGray));
            f.render_widget(banner_widget, banner_area);
        })?;

        if event::poll(std::time::Duration::from_millis(100))?
            && let event::Event::Key(key) = event::read()? {
            let should_quit = event_handler::handle_key_event(engine, key.code, key.modifiers, key.kind);
            if should_quit {
                break;
            }
        }
    }

    execute!(terminal.backend_mut(), crossterm::event::PopKeyboardEnhancementFlags)?;
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
