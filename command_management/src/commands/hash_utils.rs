pub fn is_hash(hash: &str) -> bool {
    i64::from_str_radix(hash, 16).is_ok()
}
