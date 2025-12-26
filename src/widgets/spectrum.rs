use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
};

pub struct Spectrum<'a> {
    pub buffer: &'a Arc<Mutex<VecDeque<f32>>>,
    pub sample_rate: u32,
}

#[derive(Clone, Copy, Default)]
struct Complex {
    re: f32,
    im: f32,
}

impl Complex {
    fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

fn reverse_bits(mut x: usize, bits: u32) -> usize {
    let mut y = 0usize;
    for _ in 0..bits {
        y = (y << 1) | (x & 1);
        x >>= 1;
    }
    y
}

fn fft_in_place(buf: &mut [Complex]) {
    let n = buf.len();
    if n <= 1 {
        return;
    }

    let bits = (n as f32).log2() as u32;

    // Bit-reversal.
    for i in 0..n {
        let j = reverse_bits(i, bits);
        if j > i {
            buf.swap(i, j);
        }
    }

    let mut len = 2;
    while len <= n {
        let half = len / 2;
        let ang = -2.0 * std::f32::consts::PI / (len as f32);

        for i in (0..n).step_by(len) {
            for j in 0..half {
                let (s, c) = (ang * (j as f32)).sin_cos();
                let w = Complex { re: c, im: s };

                let u = buf[i + j];
                let v = buf[i + j + half].mul(w);

                buf[i + j] = Complex {
                    re: u.re + v.re,
                    im: u.im + v.im,
                };
                buf[i + j + half] = Complex {
                    re: u.re - v.re,
                    im: u.im - v.im,
                };
            }
        }

        len *= 2;
    }
}

fn hann(i: usize, n: usize) -> f32 {
    if n <= 1 {
        return 1.0;
    }
    0.5 - 0.5 * (2.0 * std::f32::consts::PI * (i as f32) / ((n - 1) as f32)).cos()
}

fn next_pow2(mut n: usize) -> usize {
    if n < 2 {
        return 1;
    }
    n -= 1;
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    if usize::BITS == 64 {
        n |= n >> 32;
    }
    n + 1
}

fn log_bin_edges(f_min: f32, f_max: f32, bins: usize) -> Vec<(f32, f32)> {
    let ln_min = f_min.ln();
    let ln_max = f_max.ln();

    (0..bins)
        .map(|i| {
            let a = i as f32 / bins as f32;
            let b = (i + 1) as f32 / bins as f32;
            let f0 = (ln_min + a * (ln_max - ln_min)).exp();
            let f1 = (ln_min + b * (ln_max - ln_min)).exp();
            (f0, f1)
        })
        .collect()
}

impl Widget for Spectrum<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner = Block::default()
            .title("spectrum (log)")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::DarkGray))
            .inner(area);

        Block::default()
            .title("spectrum (log)")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::DarkGray))
            .render(area, buf);

        if inner.width < 2 || inner.height < 2 {
            return;
        }

        let samples = self.buffer.lock().unwrap();
        if samples.len() < 64 {
            return;
        }

        let nyquist = self.sample_rate as f32 / 2.0;
        let f_min = 20.0f32;
        let f_max = nyquist.max(f_min + 1.0);

        // FFT size: power-of-two window from the most recent samples.
        let max_n = 8192usize;
        let n = next_pow2(samples.len().min(max_n));
        let n = n.min(samples.len());
        let start = samples.len() - n;

        let mut fft_buf: Vec<Complex> = (0..n)
            .map(|i| {
                let w = hann(i, n);
                Complex {
                    re: samples[start + i] * w,
                    im: 0.0,
                }
            })
            .collect();

        fft_in_place(&mut fft_buf);

        let half = n / 2;
        let mut mags: Vec<f32> = Vec::with_capacity(half);
        let mut peak_mag = 0.0f32;
        let mut peak_bin = 0usize;

        #[allow(clippy::needless_range_loop)]
        for k in 1..half {
            let c = fft_buf[k];
            let mag = (c.re * c.re + c.im * c.im).sqrt();
            if mag > peak_mag {
                peak_mag = mag;
                peak_bin = k;
            }
            mags.push(mag);
        }

        if peak_mag <= 0.0 {
            return;
        }

        // Convert magnitudes to dB relative to peak.
        let rel_db: Vec<f32> = mags
            .iter()
            .map(|m| 20.0 * (m / peak_mag + 1e-12).log10())
            .collect();

        let bins = inner.width as usize;
        let edges = log_bin_edges(f_min, f_max, bins);

        let min_db = -80.0f32;
        let height_units = inner.height * 8;
        let blocks: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

        for (x_off, (f0, f1)) in edges.into_iter().enumerate() {
            let mut b0 = ((f0 * n as f32) / self.sample_rate as f32).floor() as isize;
            let mut b1 = ((f1 * n as f32) / self.sample_rate as f32).ceil() as isize;

            b0 = b0.clamp(1, (half - 1) as isize);
            b1 = b1.clamp(1, (half - 1) as isize);
            if b1 <= b0 {
                b1 = (b0 + 1).min((half - 1) as isize);
            }

            // Pick the max bin in the band for a readable peak.
            let mut band_db = min_db;
            for bin in (b0 as usize)..(b1 as usize) {
                // rel_db is indexed from k=1..
                let db = rel_db[bin - 1];
                if db > band_db {
                    band_db = db;
                }
            }

            let norm = ((band_db - min_db) / (0.0 - min_db)).clamp(0.0, 1.0);
            let bar_units = (norm * (height_units.saturating_sub(1) as f32)).round() as u16;
            if bar_units == 0 {
                continue;
            }

            let x = inner.x + x_off as u16;
            let mut remaining = bar_units;
            for row in 0..inner.height {
                let y = inner.y + (inner.height - 1 - row);

                if remaining == 0 {
                    break;
                }

                let level = remaining.min(8);
                remaining = remaining.saturating_sub(8);

                let ch = blocks[(level - 1) as usize];
                let color = if norm > 0.75 {
                    Color::Red
                } else if norm > 0.45 {
                    Color::Yellow
                } else {
                    Color::Cyan
                };

                buf[(x, y)].set_char(ch).set_fg(color);
            }
        }

        // Frequency hint + peak.
        let peak_hz = peak_bin as f32 * self.sample_rate as f32 / n as f32;
        let label = format!("{:.0}Hz..{:.0}Hz  peak:{:.0}Hz", f_min, nyquist, peak_hz);
        if inner.width > label.len() as u16 + 2 {
            buf.set_string(
                inner.x + 1,
                inner.y + inner.height - 1,
                label,
                Style::default().fg(Color::DarkGray),
            );
        }
    }
}
