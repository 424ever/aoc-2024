pub fn concat_numbers<const R: u64>(mut a: u64, b: u64) -> u64 {
    let mut t = b;
    while t > 0 {
        t /= R;
        a *= R;
    }
    a + b
}
