use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
};

pub struct PianoKey {
    pub label: String,
    pub color: Color,
    pub pressed: bool,
}

impl Widget for PianoKey {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = if self.pressed {
            Style::default().bg(self.color).fg(Color::Black)
        } else {
            Style::default().bg(Color::Black).fg(self.color)
        };

        let block = Block::default().borders(Borders::ALL).style(style);
        block.render(area, buf);

        let text_width = self.label.len() as u16;
        let x = area.x + (area.width.saturating_sub(text_width)) / 2;
        let y = area.y + area.height / 2;
        buf.set_string(x, y, &self.label, style);
    }
}
