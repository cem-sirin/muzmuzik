use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
};

/// A custom widget representing a piano key.
pub struct PianoKey {
    pub note: char,
    pub color: Color,
    pub pressed: bool,
}

impl Widget for PianoKey {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Style: bright background when pressed, dark otherwise
        let style = if self.pressed {
            Style::default().bg(self.color).fg(Color::Black)
        } else {
            Style::default().bg(Color::Black).fg(self.color)
        };

        // Render block with borders
        let block = Block::default().borders(Borders::ALL).style(style);
        block.render(area, buf);

        // Center the note text
        let text = self.note.to_string();
        let text_width = text.len() as u16;
        let x = area.x + (area.width.saturating_sub(text_width)) / 2;
        let y = area.y + area.height / 2;
        buf.set_string(x, y, &text, style);
    }
}

/// Convert HSL to RGB.
/// h: hue (0-360), s: saturation (0-1), l: lightness (0-1)
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = match (h / 60.0) as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        5 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };
    (
        ((r + m).clamp(0.0, 1.0) * 255.0) as u8,
        ((g + m).clamp(0.0, 1.0) * 255.0) as u8,
        ((b + m).clamp(0.0, 1.0) * 255.0) as u8,
    )
}

/// Get a unique color for a note based on its index using the angle formula.
pub fn note_color(index: usize) -> Color {
    let hue = (index as f32 * 210.0) % 360.0;
    let (r, g, b) = hsl_to_rgb(hue, 1.0, 0.5);
    Color::Rgb(r, g, b)
}