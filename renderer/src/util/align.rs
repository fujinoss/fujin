#[inline]
pub const fn align_up(value: usize, alignment: usize) -> usize {
    debug_assert!(alignment.is_power_of_two());
    (value + alignment - 1) & !(alignment - 1)
}

#[inline]
pub const fn align_down(value: usize, alignment: usize) -> usize {
    debug_assert!(alignment.is_power_of_two());
    value & !(alignment - 1)
}

#[inline]
pub const fn is_aligned(value: usize, alignment: usize) -> bool {
    debug_assert!(alignment.is_power_of_two());
    value & (alignment - 1) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_up_boundaries() {
        assert_eq!(align_up(0, 4), 0);
        assert_eq!(align_up(1, 4), 4);
        assert_eq!(align_up(3, 4), 4);
        assert_eq!(align_up(4, 4), 4);
        assert_eq!(align_up(5, 4), 8);
        assert_eq!(align_up(16, 16), 16);
        assert_eq!(align_up(17, 16), 32);
        assert_eq!(align_up(31, 16), 32);
        assert_eq!(align_up(32, 16), 32);
    }

    #[test]
    fn test_align_down_boundaries() {
        assert_eq!(align_down(0, 4), 0);
        assert_eq!(align_down(1, 4), 0);
        assert_eq!(align_down(3, 4), 0);
        assert_eq!(align_down(4, 4), 4);
        assert_eq!(align_down(7, 4), 4);
        assert_eq!(align_down(15, 16), 0);
        assert_eq!(align_down(16, 16), 16);
        assert_eq!(align_down(31, 16), 16);
        assert_eq!(align_down(32, 16), 32);
    }

    #[test]
    fn test_is_aligned() {
        assert!(is_aligned(0, 4));
        assert!(!is_aligned(1, 4));
        assert!(!is_aligned(3, 4));
        assert!(is_aligned(4, 4));
        assert!(is_aligned(8, 4));
        assert!(!is_aligned(15, 16));
        assert!(is_aligned(16, 16));
        assert!(is_aligned(1024, 256));
    }

    #[test]
    fn test_align_up_then_down_returns_to_alignment() {
        for align in [2usize, 4, 8, 16, 32, 64, 128, 256] {
            for value in 0..(align * 4) {
                let up = align_up(value, align);
                let down = align_down(up, align);
                assert_eq!(up, down);
            }
        }
    }
}
