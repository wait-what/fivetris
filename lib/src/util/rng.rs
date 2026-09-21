const MAGIC: u64 = 0x9E3779B97F4A7C15; // golden ratio

#[inline]
pub fn next(mut state: u64) -> u64 {
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;
    state = state.wrapping_add(MAGIC);
    state
}

#[inline]
#[allow(dead_code)] // TODO:
pub fn previous(mut state: u64) -> u64 {
    state = state.wrapping_sub(MAGIC);
    state = unxorshift_left(state, 17);
    state = unxorshift_right(state, 7);
    state = unxorshift_left(state, 13);
    state
}

#[inline]
fn unxorshift_left(x: u64, shift: u32) -> u64 {
    match shift {
        13 => x ^ x << 13 ^ x << 26 ^ x << 39 ^ x << 52,
        17 => x ^ x << 17 ^ x << 34 ^ x << 51,
        _ => unreachable!()
    }
}

#[inline]
fn unxorshift_right(x: u64, shift: u32) -> u64 {
    match shift {
        7 => x ^ x >> 7 ^ x >> 14 ^ x >> 21 ^ x >> 28 ^ x >> 35 ^ x >> 42 ^ x >> 49 ^ x >> 56,
        _ => unreachable!()
    }
}
