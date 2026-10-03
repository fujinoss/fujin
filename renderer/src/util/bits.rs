#[inline]
pub const fn is_power_of_two(n: u32) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

#[inline]
pub const fn round_up_to_pow2(n: u32, pow2: u32) -> u32 {
    debug_assert!(is_power_of_two(pow2));
    (n + pow2 - 1) & !(pow2 - 1)
}

#[inline]
pub const fn round_down_to_pow2(n: u32, pow2: u32) -> u32 {
    debug_assert!(is_power_of_two(pow2));
    n & !(pow2 - 1)
}

#[inline]
pub const fn next_power_of_two(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }
    let mut v = n - 1;
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v + 1
}

#[inline]
pub const fn popcount(n: u64) -> u32 {
    n.count_ones()
}

#[inline]
pub const fn leading_zeros(n: u64) -> u32 {
    n.leading_zeros()
}

#[inline]
pub const fn trailing_zeros(n: u64) -> u32 {
    n.trailing_zeros()
}

#[inline]
pub const fn align_up_u64(value: u64, alignment: u64) -> u64 {
    debug_assert!(alignment.is_power_of_two());
    (value + alignment - 1) & !(alignment - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_two_true_cases() {
        for n in [1u32, 2, 4, 8, 16, 32, 64, 128, 256, 1024, 1 << 20] {
            assert!(is_power_of_two(n), "{n} should be power of two");
        }
    }

    #[test]
    fn test_is_power_of_two_false_cases() {
        for n in [0u32, 3, 5, 6, 7, 9, 15, 17, 100, 1023] {
            assert!(!is_power_of_two(n), "{n} should not be power of two");
        }
    }

    #[test]
    fn test_round_up_to_pow2_all_alignments() {
        for align in [1u32, 2, 4, 8, 16, 32, 64] {
            for n in 0..(align * 4) {
                let r = round_up_to_pow2(n, align);
                assert!(r >= n);
                assert!(r % align == 0);
                assert!(r - n < align);
            }
        }
    }

    #[test]
    fn test_round_down_to_pow2_all_alignments() {
        for align in [1u32, 2, 4, 8, 16, 32, 64] {
            for n in 0..(align * 4) {
                let r = round_down_to_pow2(n, align);
                assert!(r <= n);
                assert!(r % align == 0);
                assert!(n - r < align);
            }
        }
    }

    #[test]
    fn test_next_power_of_two_sequence() {
        assert_eq!(next_power_of_two(0), 1);
        assert_eq!(next_power_of_two(1), 1);
        assert_eq!(next_power_of_two(2), 2);
        assert_eq!(next_power_of_two(3), 4);
        assert_eq!(next_power_of_two(4), 4);
        assert_eq!(next_power_of_two(5), 8);
        assert_eq!(next_power_of_two(7), 8);
        assert_eq!(next_power_of_two(8), 8);
        assert_eq!(next_power_of_two(9), 16);
        assert_eq!(next_power_of_two(17), 32);
        assert_eq!(next_power_of_two(1000), 1024);
    }

    #[test]
    fn test_popcount() {
        assert_eq!(popcount(0), 0);
        assert_eq!(popcount(1), 1);
        assert_eq!(popcount(0b1111), 4);
        assert_eq!(popcount(u64::MAX), 64);
        assert_eq!(popcount(0xAAAA_AAAA_AAAA_AAAA), 32);
    }

    #[test]
    fn test_leading_zeros() {
        assert_eq!(leading_zeros(1), 63);
        assert_eq!(leading_zeros(0x8000_0000_0000_0000), 0);
        assert_eq!(leading_zeros(0), 64);
    }

    #[test]
    fn test_trailing_zeros() {
        assert_eq!(trailing_zeros(1), 0);
        assert_eq!(trailing_zeros(2), 1);
        assert_eq!(trailing_zeros(8), 3);
        assert_eq!(trailing_zeros(0), 64);
    }

    #[test]
    fn test_align_up_u64() {
        assert_eq!(align_up_u64(0, 4), 0);
        assert_eq!(align_up_u64(1, 4), 4);
        assert_eq!(align_up_u64(4, 4), 4);
        assert_eq!(align_up_u64(5, 4), 8);
        assert_eq!(align_up_u64(1024, 256), 1024);
        assert_eq!(align_up_u64(1025, 256), 1280);
    }
}
