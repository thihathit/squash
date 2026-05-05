use humansize::{DECIMAL, format_size};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn human_size(size: u64) -> String {
    format_size(size, DECIMAL)
}

pub fn get_random_in_range(min: u64, max: u64) -> u64 {
    // 1. Seed using full nanoseconds (u128 cast to u64)
    let mut seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos() as u64;

    // 2. 64-bit Xorshift PRNG
    seed ^= seed << 13;
    seed ^= seed >> 7;
    seed ^= seed << 17;

    // 3. Constrain to range
    min + (seed % (max - min + 1))
}
