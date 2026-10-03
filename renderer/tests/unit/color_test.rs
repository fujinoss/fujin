use arashi::color::blend::{dim, lerp, multiply, screen, source_over, source_over_premultiplied};
use arashi::color::hsv::Hsv;
use arashi::color::srgb::{
    linear_to_srgb, linear_to_srgb_u8, srgb_to_linear, srgb_to_linear_u8, SrgbLut,
};
use arashi::color::{Color, Palette};

#[test]
fn test_color_pack_unpack_rgba_exhaustive() {
    for r in [0u8, 1, 127, 128, 255] {
        for g in [0u8, 1, 127, 128, 255] {
            for b in [0u8, 1, 127, 128, 255] {
                for a in [0u8, 1, 127, 128, 255] {
                    let c = Color::new(r, g, b, a);
                    let packed = c.to_u32_rgba();
                    let back = Color::from_u32_rgba(packed);
                    assert_eq!(c, back);
                }
            }
        }
    }
}

#[test]
fn test_color_pack_unpack_argb() {
    let c = Color::new(0x11, 0x22, 0x33, 0x44);
    assert_eq!(c.to_u32_argb(), 0x44112233);
    assert_eq!(Color::from_u32_argb(0x44112233), c);
}

#[test]
fn test_color_from_hex() {
    assert_eq!(Color::from_hex(0xff0000), Color::rgb(255, 0, 0));
    assert_eq!(Color::from_hex(0x00ff00), Color::rgb(0, 255, 0));
    assert_eq!(Color::from_hex(0x0000ff), Color::rgb(0, 0, 255));
    assert_eq!(Color::from_hex(0xffffff), Color::WHITE);
    assert_eq!(Color::from_hex(0x000000), Color::BLACK);
}

#[test]
fn test_color_from_hex_alpha() {
    let c = Color::from_hex_alpha(0xff8800, 128);
    assert_eq!(c, Color::new(255, 0x88, 0, 128));
}

#[test]
fn test_color_invert() {
    let c = Color::new(0, 128, 255, 200);
    let inv = c.invert();
    assert_eq!(inv, Color::new(255, 127, 0, 200));
    assert_eq!(inv.invert(), c);
}

#[test]
fn test_color_luminance() {
    assert!(Color::BLACK.luminance() < 0.01);
    assert!(Color::WHITE.luminance() > 0.99);
    assert!(Color::BLACK.is_dark());
    assert!(!Color::WHITE.is_dark());
}

#[test]
fn test_color_contrast() {
    let c = Color::WHITE.contrast_with(Color::BLACK);
    assert!(c > 20.0);
    let c2 = Color::WHITE.contrast_with(Color::WHITE);
    assert!((c2 - 1.0).abs() < 0.01);
}

#[test]
fn test_color_with_alpha() {
    let c = Color::rgb(100, 150, 200);
    let c2 = c.with_alpha(128);
    assert_eq!(c2.r, 100);
    assert_eq!(c2.g, 150);
    assert_eq!(c2.b, 200);
    assert_eq!(c2.a, 128);
}

#[test]
fn test_srgb_roundtrip_exhaustive() {
    for i in 0u8..=255 {
        let linear = srgb_to_linear_u8(i);
        let back = linear_to_srgb_u8(linear);
        let diff = (i as i32 - back as i32).abs();
        assert!(diff <= 1, "srgb roundtrip failed: {i} -> {linear} -> {back}");
    }
}

#[test]
fn test_srgb_endpoints() {
    assert_eq!(srgb_to_linear(0.0), 0.0);
    assert!((srgb_to_linear(1.0) - 1.0).abs() < 1e-6);
    assert_eq!(linear_to_srgb(0.0), 0.0);
    assert!((linear_to_srgb(1.0) - 1.0).abs() < 1e-6);
}

#[test]
fn test_srgb_lut_matches_function() {
    let lut = SrgbLut::new();
    for i in 0u8..=255 {
        let a = lut.lookup(i);
        let b = srgb_to_linear_u8(i);
        assert!((a - b).abs() < 1e-9);
    }
}

#[test]
fn test_hsv_from_rgb_primaries() {
    let red = Hsv::from_rgb(255, 0, 0);
    assert!((red.h - 0.0).abs() < 0.01);
    assert!((red.s - 1.0).abs() < 0.01);
    assert!((red.v - 1.0).abs() < 0.01);

    let green = Hsv::from_rgb(0, 255, 0);
    assert!((green.h - 120.0).abs() < 0.01);

    let blue = Hsv::from_rgb(0, 0, 255);
    assert!((blue.h - 240.0).abs() < 0.01);
}

#[test]
fn test_hsv_roundtrip_exhaustive() {
    for r in [0u8, 32, 64, 96, 128, 160, 192, 224, 255] {
        for g in [0u8, 32, 64, 96, 128, 160, 192, 224, 255] {
            for b in [0u8, 32, 64, 96, 128, 160, 192, 224, 255] {
                let (r2, g2, b2) = Hsv::from_rgb(r, g, b).to_rgb();
                assert!((r as i32 - r2 as i32).abs() <= 1, "r mismatch {r} {r2}");
                assert!((g as i32 - g2 as i32).abs() <= 1, "g mismatch {g} {g2}");
                assert!((b as i32 - b2 as i32).abs() <= 1, "b mismatch {b} {b2}");
            }
        }
    }
}

#[test]
fn test_hsv_gray() {
    let gray = Hsv::from_rgb(128, 128, 128);
    assert!((gray.s - 0.0).abs() < 0.01);
    assert!((gray.v - 128.0 / 255.0).abs() < 0.01);
}

#[test]
fn test_blend_source_over_opaque_replaces() {
    let out = source_over(Color::RED, Color::BLUE);
    assert_eq!(out, Color::RED);
}

#[test]
fn test_blend_source_over_transparent_keeps_dst() {
    let src = Color::TRANSPARENT;
    let dst = Color::BLUE;
    let out = source_over(src, dst);
    assert_eq!(out, dst);
}

#[test]
fn test_blend_lerp_endpoints() {
    let a = Color::new(10, 20, 30, 40);
    let b = Color::new(200, 210, 220, 230);
    assert_eq!(lerp(a, b, 0.0), a);
    assert_eq!(lerp(a, b, 1.0), b);
}

#[test]
fn test_blend_lerp_midpoint() {
    let mid = lerp(Color::BLACK, Color::WHITE, 0.5);
    assert_eq!(mid.r, 128);
    assert_eq!(mid.g, 128);
    assert_eq!(mid.b, 128);
    assert_eq!(mid.a, 255);
}

#[test]
fn test_blend_dim() {
    let c = Color::WHITE;
    assert_eq!(dim(c, 0.0), Color::BLACK);
    assert_eq!(dim(c, 1.0), Color::WHITE);
    let half = dim(c, 0.5);
    assert_eq!(half.r, 127);
}

#[test]
fn test_blend_multiply_with_white_keeps() {
    let a = Color::rgb(100, 150, 200);
    let out = multiply(a, Color::WHITE);
    assert_eq!(out.r, 100);
    assert_eq!(out.g, 150);
    assert_eq!(out.b, 200);
}

#[test]
fn test_blend_multiply_with_black_blackens() {
    let a = Color::rgb(100, 150, 200);
    let out = multiply(a, Color::BLACK);
    assert_eq!(out.r, 0);
    assert_eq!(out.g, 0);
    assert_eq!(out.b, 0);
}

#[test]
fn test_blend_screen_with_black_keeps() {
    let a = Color::rgb(100, 150, 200);
    let out = screen(a, Color::BLACK);
    assert_eq!(out.r, 100);
    assert_eq!(out.g, 150);
    assert_eq!(out.b, 200);
}

#[test]
fn test_blend_screen_with_white_whitens() {
    let a = Color::rgb(100, 150, 200);
    let out = screen(a, Color::WHITE);
    assert_eq!(out.r, 255);
    assert_eq!(out.g, 255);
    assert_eq!(out.b, 255);
}

#[test]
fn test_blend_premultiplied_transparent_is_identity() {
    let src = Color::TRANSPARENT;
    let dst = Color::rgb(100, 150, 200);
    let out = source_over_premultiplied(src, dst);
    assert_eq!(out.r, 100);
    assert_eq!(out.g, 150);
    assert_eq!(out.b, 200);
}

#[test]
fn test_palette_ansi_range() {
    let p = Palette::default();
    for i in 0u8..16 {
        assert_eq!(p.ansi(i), p.colors[i as usize]);
    }
    assert_eq!(p.ansi(16), p.colors[0]);
    assert_eq!(p.ansi(255), p.colors[15]);
}

#[test]
fn test_palette_256_cube_corners() {
    let p = Palette::default();
    assert_eq!(p.indexed(16), Color::rgb(0, 0, 0));
    assert_eq!(p.indexed(21), Color::rgb(0, 0, 255));
    assert_eq!(p.indexed(196), Color::rgb(255, 0, 0));
    assert_eq!(p.indexed(231), Color::rgb(255, 255, 255));
}

#[test]
fn test_palette_256_grayscale() {
    let p = Palette::default();
    assert_eq!(p.indexed(232), Color::rgb(8, 8, 8));
    assert_eq!(p.indexed(255), Color::rgb(238, 238, 238));
    let prev = p.indexed(232);
    for i in 233u8..=255 {
        let curr = p.indexed(i);
        assert!(curr.r > prev.r, "grayscale must increase");
    }
}

#[test]
fn test_palette_default_is_dark() {
    let a = Palette::default();
    let b = Palette::default_dark();
    for i in 0..16 {
        assert_eq!(a.colors[i], b.colors[i]);
    }
}
