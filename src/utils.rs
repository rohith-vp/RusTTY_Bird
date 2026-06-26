use crossterm::style::Color;


/// Darkens a crossterm Color.
/// `factor` should be between 0.0 (completely black) and 1.0 (no change).
pub fn darken_color(color: Color, factor: f32) -> Color {
    // Clamp the factor to guarantee safety and avoid unexpected overflow artifacts
    let factor = factor.clamp(0.0, 1.0);

    match color {
        // If it's an RGB color, mathematically scale the channels down
        Color::Rgb { r, g, b } => Color::Rgb {
            r: (r as f32 * factor).round() as u8,
            g: (g as f32 * factor).round() as u8,
            b: (b as f32 * factor).round() as u8,
        },

        // If it's a standard ANSI color, manually map it to its darker variant
        Color::AnsiValue(val) => Color::AnsiValue(darken_ansi_value(val)),

        // Fallback or explicit mapping for named enum variants
        Color::Blue => Color::DarkBlue,
        Color::Green => Color::DarkGreen,
        Color::Cyan => Color::DarkCyan,
        Color::Magenta => Color::DarkMagenta,
        Color::Red => Color::DarkRed,
        Color::Yellow => Color::AnsiValue(3), // ANSI 3 is Olive/Dark Yellow

        // If it's already dark, grayscale, or Reset, leave it as-is
        other => other,
    }
}


// Optional helper if your project relies on raw 8-bit ANSI colors (0-255)
fn darken_ansi_value(val: u8) -> u8 {
    // Simplistic safe fallback: if it's in the standard 16-color block, shift it down
    if val >= 8 && val <= 15 { val - 8 } else { val }
}
