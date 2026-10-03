#[inline]
pub fn srgb_to_linear(s: f32) -> f32 {
    if s <= 0.04045 {
        s / 12.92
    } else {
        ((s + 0.055) / 1.055).powf(2.4)
    }
}

#[inline]
pub fn linear_to_srgb(l: f32) -> f32 {
    if l <= 0.0031308 {
        l * 12.92
    } else {
        1.055 * l.powf(1.0 / 2.4) - 0.055
    }
}

#[inline]
pub fn srgb_to_linear_u8(c: u8) -> f32 {
    srgb_to_linear(c as f32 / 255.0)
}

#[inline]
pub fn linear_to_srgb_u8(l: f32) -> u8 {
    (linear_to_srgb(l).clamp(0.0, 1.0) * 255.0 + 0.5) as u8
}

pub struct SrgbLut {
    table: [f32; 256],
}

impl SrgbLut {
    pub fn new() -> Self {
        let mut table = [0.0f32; 256];
        let mut i = 0;
        while i < 256 {
            table[i] = srgb_to_linear_u8(i as u8);
            i += 1;
        }
        Self { table }
    }

    #[inline]
    pub fn lookup(&self, c: u8) -> f32 {
        self.table[c as usize]
    }
}

impl Default for SrgbLut {
    fn default() -> Self {
        Self::new()
    }
}
