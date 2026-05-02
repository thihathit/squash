use humansize::{DECIMAL, format_size};

pub fn human_size(size: u64) -> String {
    format_size(size, DECIMAL)
}
