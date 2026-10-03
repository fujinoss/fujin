use super::rgba::Color;

#[inline]
pub fn source_over(src: Color, dst: Color) -> Color {
    let sa = src.a as f32 / 255.0;
    let da = dst.a as f32 / 255.0;
    let out_a = sa + da * (1.0 - sa);
    if out_a == 0.0 {
        return Color::TRANSPARENT;
    }
    let r = (src.r as f32 * sa + dst.r as f32 * da * (1.0 - sa)) / out_a;
    let g = (src.g as f32 * sa + dst.g as f32 * da * (1.0 - sa)) / out_a;
    let b = (src.b as f32 * sa + dst.b as f32 * da * (1.0 - sa)) / out_a;
    Color {
        r: r.clamp(0.0, 255.0) as u8,
        g: g.clamp(0.0, 255.0) as u8,
        b: b.clamp(0.0, 255.0) as u8,
        a: (out_a * 255.0).clamp(0.0, 255.0) as u8,
    }
}

#[inline]
pub fn source_over_premultiplied(src: Color, dst: Color) -> Color {
    let sa = src.a as f32 / 255.0;
    let inv = 1.0 - sa;
    let r = (src.r as f32 + dst.r as f32 * inv).min(255.0);
    let g = (src.g as f32 + dst.g as f32 * inv).min(255.0);
    let b = (src.b as f32 + dst.b as f32 * inv).min(255.0);
    let a = (src.a as f32 + dst.a as f32 * inv).min(255.0);
    Color {
        r: r as u8,
        g: g as u8,
        b: b as u8,
        a: a as u8,
    }
}

#[inline]
pub fn multiply(a: Color, b: Color) -> Color {
    Color {
        r: ((a.r as u16 * b.r as u16) / 255) as u8,
        g: ((a.g as u16 * b.g as u16) / 255) as u8,
        b: ((a.b as u16 * b.b as u16) / 255) as u8,
        a: a.a,
    }
}

#[inline]
pub fn screen(a: Color, b: Color) -> Color {
    Color {
        r: 255 - ((255 - a.r as u16) * (255 - b.r as u16) / 255) as u8,
        g: 255 - ((255 - a.g as u16) * (255 - b.g as u16) / 255) as u8,
        b: 255 - ((255 - a.b as u16) * (255 - b.b as u16) / 255) as u8,
        a: a.a.max(b.a),
    }
}

#[inline]
pub fn lerp(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    let inv = 1.0 - t;
    Color {
        r: (a.r as f32 * inv + b.r as f32 * t + 0.5) as u8,
        g: (a.g as f32 * inv + b.g as f32 * t + 0.5) as u8,
        b: (a.b as f32 * inv + b.b as f32 * t + 0.5) as u8,
        a: (a.a as f32 * inv + b.a as f32 * t + 0.5) as u8,
    }
}

#[inline]
pub fn dim(c: Color, factor: f32) -> Color {
    let f = factor.clamp(0.0, 1.0);
    Color {
        r: (c.r as f32 * f) as u8,
        g: (c.g as f32 * f) as u8,
        b: (c.b as f32 * f) as u8,
        a: c.a,
    }
}
