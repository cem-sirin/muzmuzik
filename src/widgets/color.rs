use ratatui::style::Color;

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

pub fn note_color(index: usize) -> Color {
    let hue = (index as f32 * 210.0) % 360.0;
    let (r, g, b) = hsl_to_rgb(hue, 1.0, 0.5);
    Color::Rgb(r, g, b)
}
