use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
};

pub struct Waveform<'a> {
    pub buffer: &'a Arc<Mutex<VecDeque<f32>>>,
}

fn braille_dot_bit(x: u16, y: u16) -> u8 {
    // Braille is 2x4 dots.
    // (0,0)=1 (0,1)=2 (0,2)=3 (1,0)=4 (1,1)=5 (1,2)=6 (0,3)=7 (1,3)=8
    match (x, y) {
        (0, 0) => 0,
        (0, 1) => 1,
        (0, 2) => 2,
        (1, 0) => 3,
        (1, 1) => 4,
        (1, 2) => 5,
        (0, 3) => 6,
        (1, 3) => 7,
        _ => 0,
    }
}

fn plot_braille_point(cells: &mut [u8], width: u16, x: i32, y: i32) {
    if width == 0 {
        return;
    }

    if x < 0 || y < 0 {
        return;
    }

    let x = x as u16;
    let y = y as u16;

    let cell_x = x / 2;
    let cell_y = y / 4;
    let sub_x = x % 2;
    let sub_y = y % 4;

    let idx = (cell_y as usize) * (width as usize) + (cell_x as usize);
    if idx >= cells.len() {
        return;
    }

    let bit = braille_dot_bit(sub_x, sub_y);
    cells[idx] |= 1u8 << bit;
}

fn plot_braille_line(cells: &mut [u8], width: u16, x0: i32, y0: i32, x1: i32, y1: i32) {
    // Bresenham on the sub-cell grid.
    let mut x0 = x0;
    let mut y0 = y0;

    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        plot_braille_point(cells, width, x0, y0);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

impl Widget for Waveform<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("waveform")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        if inner.width < 2 || inner.height < 2 {
            return;
        }

        let samples = self.buffer.lock().unwrap();
        if samples.len() < 2 {
            return;
        }

        // 2x4 subcells per terminal cell.
        // We draw one polyline across the entire width using recent samples.
        let grid_w = inner.width * 2;
        let grid_h = inner.height * 4;
        if grid_w < 2 || grid_h < 2 {
            return;
        }

        let points = grid_w as usize;
        let available = samples.len();

        // Time zoom: show a shorter window so wave periods are readable.
        // (Lower step => fewer samples shown => longer apparent wavelength.)
        let target_step: usize = 16;
        let window_len = (points * target_step).min(available);
        let start = available - window_len;
        let step = (window_len / points).max(1);

        // Auto-scale amplitude to use the vertical range.
        let mut peak = 0.0f32;
        for s in samples.iter().skip(start) {
            peak = peak.max(s.abs());
        }
        let peak = peak.max(1e-3);

        let mut cells = vec![0u8; (inner.width * inner.height) as usize];

        let mut prev_x = 0i32;
        let mut prev_y = (grid_h as i32) / 2;

        for i in 0..points {
            let src = start + i * step;
            if src >= available {
                break;
            }

            let v = (samples[src] / peak).clamp(-1.0, 1.0);
            let y = ((1.0 - (v + 1.0) * 0.5) * (grid_h.saturating_sub(1) as f32)).round() as i32;
            let x = i as i32;

            if i > 0 {
                plot_braille_line(&mut cells, inner.width, prev_x, prev_y, x, y);
            } else {
                plot_braille_point(&mut cells, inner.width, x, y);
            }

            prev_x = x;
            prev_y = y;
        }

        // Render braille chars.
        for row in 0..inner.height {
            for col in 0..inner.width {
                let idx = (row as usize) * (inner.width as usize) + (col as usize);
                let bits = cells[idx];
                if bits == 0 {
                    continue;
                }

                let ch = char::from_u32(0x2800 + bits as u32).unwrap_or(' ');
                let x = inner.x + col;
                let y = inner.y + row;
                buf[(x, y)].set_char(ch).set_fg(Color::Green);
            }
        }
    }
}
