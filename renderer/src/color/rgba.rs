#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const TRANSPARENT: Self = Self::new(0, 0, 0, 0);
    pub const BLACK: Self = Self::new(0, 0, 0, 255);
    pub const WHITE: Self = Self::new(255, 255, 255, 255);
    pub const RED: Self = Self::new(255, 0, 0, 255);
    pub const GREEN: Self = Self::new(0, 255, 0, 255);
    pub const BLUE: Self = Self::new(0, 0, 255, 255);
    pub const YELLOW: Self = Self::new(255, 255, 0, 255);
    pub const CYAN: Self = Self::new(0, 255, 255, 255);
    pub const MAGENTA: Self = Self::new(255, 0, 255, 255);

    #[inline]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    #[inline]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    #[inline]
    pub const fn gray(v: u8) -> Self {
        Self::rgb(v, v, v)
    }

    #[inline]
    pub const fn from_u32_rgba(v: u32) -> Self {
        Self {
            r: ((v >> 24) & 0xFF) as u8,
            g: ((v >> 16) & 0xFF) as u8,
            b: ((v >> 8) & 0xFF) as u8,
            a: (v & 0xFF) as u8,
        }
    }

    #[inline]
    pub const fn to_u32_rgba(self) -> u32 {
        ((self.r as u32) << 24)
            | ((self.g as u32) << 16)
            | ((self.b as u32) << 8)
            | (self.a as u32)
    }

    #[inline]
    pub const fn from_u32_argb(v: u32) -> Self {
        Self {
            a: ((v >> 24) & 0xFF) as u8,
            r: ((v >> 16) & 0xFF) as u8,
            g: ((v >> 8) & 0xFF) as u8,
            b: (v & 0xFF) as u8,
        }
    }

    #[inline]
    pub const fn to_u32_argb(self) -> u32 {
        ((self.a as u32) << 24)
            | ((self.r as u32) << 16)
            | ((self.g as u32) << 8)
            | (self.b as u32)
    }

    #[inline]
    pub const fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
            a: 255,
        }
    }

    #[inline]
    pub const fn from_hex_alpha(hex: u32, alpha: u8) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
            a: alpha,
        }
    }

    #[inline]
    pub fn to_srgb_f32(self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        ]
    }

    #[inline]
    pub fn to_linear_f32(self) -> [f32; 4] {
        [
            super::srgb::srgb_to_linear_u8(self.r),
            super::srgb::srgb_to_linear_u8(self.g),
            super::srgb::srgb_to_linear_u8(self.b),
            self.a as f32 / 255.0,
        ]
    }

    #[inline]
    pub fn with_alpha(self, alpha: u8) -> Self {
        Self { a: alpha, ..self }
    }

    #[inline]
    pub fn invert(self) -> Self {
        Self {
            r: 255 - self.r,
            g: 255 - self.g,
            b: 255 - self.b,
            a: self.a,
        }
    }

    #[inline]
    pub fn luminance(self) -> f32 {
        0.2126 * super::srgb::srgb_to_linear_u8(self.r)
            + 0.7152 * super::srgb::srgb_to_linear_u8(self.g)
            + 0.0722 * super::srgb::srgb_to_linear_u8(self.b)
    }

    #[inline]
    pub fn is_dark(self) -> bool {
        self.luminance() < 0.5
    }

    #[inline]
    pub fn contrast_with(self, other: Color) -> f32 {
        let l1 = self.luminance();
        let l2 = other.luminance();
        let (bright, dark) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
        (bright + 0.05) / (dark + 0.05)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::TRANSPARENT
    }
}
