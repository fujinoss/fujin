use super::rgba::Color;

pub const ANSI_COUNT: usize = 16;

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub colors: [Color; ANSI_COUNT],
}

impl Palette {
    pub const fn default_dark() -> Self {
        Self {
            colors: [
                Color::from_hex(0x0e0e12),
                Color::from_hex(0xe04b4b),
                Color::from_hex(0x88c057),
                Color::from_hex(0xe0b04b),
                Color::from_hex(0x4b8de0),
                Color::from_hex(0xb04be0),
                Color::from_hex(0x4be0c0),
                Color::from_hex(0xd8d8d8),
                Color::from_hex(0x1e1e24),
                Color::from_hex(0xff6b6b),
                Color::from_hex(0xa8e077),
                Color::from_hex(0xffd06b),
                Color::from_hex(0x6ba8ff),
                Color::from_hex(0xd06bff),
                Color::from_hex(0x6bffe0),
                Color::from_hex(0xffffff),
            ],
        }
    }

    pub const fn default_light() -> Self {
        Self {
            colors: [
                Color::from_hex(0x1a1a1a),
                Color::from_hex(0xcc0000),
                Color::from_hex(0x4e9a06),
                Color::from_hex(0xc4a000),
                Color::from_hex(0x3465a4),
                Color::from_hex(0x75507b),
                Color::from_hex(0x06989a),
                Color::from_hex(0xd3d7cf),
                Color::from_hex(0x555753),
                Color::from_hex(0xef2929),
                Color::from_hex(0x8ae234),
                Color::from_hex(0xfce94f),
                Color::from_hex(0x729fcf),
                Color::from_hex(0xad7fa8),
                Color::from_hex(0x34e2e2),
                Color::from_hex(0xeeeeec),
            ],
        }
    }

    #[inline]
    pub fn ansi(&self, index: u8) -> Color {
        self.colors[(index as usize) % ANSI_COUNT]
    }

    pub fn indexed(&self, index: u8) -> Color {
        if index < 16 {
            return self.colors[index as usize];
        }
        if index < 232 {
            let i = index - 16;
            let r = i / 36;
            let g = (i % 36) / 6;
            let b = i % 6;
            let to_byte = |c: u8| -> u8 {
                if c == 0 {
                    0
                } else {
                    55 + c * 40
                }
            };
            Color::rgb(to_byte(r), to_byte(g), to_byte(b))
        } else {
            let gray = 8 + (index - 232) * 10;
            Color::rgb(gray, gray, gray)
        }
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::default_dark()
    }
}
