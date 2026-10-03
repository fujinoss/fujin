use super::rgba::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsv {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

impl Hsv {
    #[inline]
    pub const fn new(h: f32, s: f32, v: f32) -> Self {
        Self { h, s, v }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let rf = r as f32 / 255.0;
        let gf = g as f32 / 255.0;
        let bf = b as f32 / 255.0;

        let max = rf.max(gf).max(bf);
        let min = rf.min(gf).min(bf);
        let delta = max - min;

        let h = if delta == 0.0 {
            0.0
        } else if (max - rf).abs() < f32::EPSILON {
            60.0 * (((gf - bf) / delta) % 6.0)
        } else if (max - gf).abs() < f32::EPSILON {
            60.0 * ((bf - rf) / delta + 2.0)
        } else {
            60.0 * ((rf - gf) / delta + 4.0)
        };

        let h = if h < 0.0 { h + 360.0 } else { h };
        let s = if max == 0.0 { 0.0 } else { delta / max };

        Self { h, s, v: max }
    }

    pub fn to_rgb(self) -> (u8, u8, u8) {
        let h = self.h.rem_euclid(360.0);
        let c = self.v * self.s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = self.v - c;

        let (r1, g1, b1) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        (
            ((r1 + m) * 255.0 + 0.5) as u8,
            ((g1 + m) * 255.0 + 0.5) as u8,
            ((b1 + m) * 255.0 + 0.5) as u8,
        )
    }

    pub fn to_color(self) -> Color {
        let (r, g, b) = self.to_rgb();
        Color::rgb(r, g, b)
    }
}
