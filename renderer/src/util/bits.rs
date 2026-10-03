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
    fn test_is_power_of_two() {
        assert!(is_power_of_two(1));
        assert!(is_power_of_two(2));
        assert!(is_power_of_two(1024));
        assert!(!is_power_of_two(0));
        assert!(!is_power_of_two(3));
        assert!(!is_power_of_two(1023));
    }

    #[test]
    fn test_round_up_to_pow2() {
        assert_eq!(round_up_to_pow2(5, 4), 8);
        assert_eq!(round_up_to_pow2(8, 8), 8);
        assert_eq!(round_up_to_pow2(9, 8), 16);
        assert_eq!(round_up_to_pow2(1, 16), 16);
    }

    #[test]
    fn test_round_down_to_pow2() {
        assert_eq!(round_down_to_pow2(5, 4), 4);
        assert_eq!(round_down_to_pow2(8, 8), 8);
        assert_eq!(round_down_to_pow2(9, 8), 8);
        assert_eq!(round_down_to_pow2(15, 16), 0);
    }

    #[test]
    fn test_next_power_of_two() {
        assert_eq!(next_power_of_two(0), 1);
        assert_eq!(next_power_of_two(1), 1);
        assert_eq!(next_power_of_two(2), 2);
        assert_eq!(next_power_of_two(3), 4);
        assert_eq!(next_power_of_two(5), 8);
        assert_eq!(next_power_of_two(17), 32);
    }
}
