pub fn concat_numbers<const R: u64>(mut a: u64, b: u64) -> u64 {
    let mut t = b;
    while t > 0 {
        t /= R;
        a *= R;
    }
    a + b
}

pub fn count_digits<const R: u64>(n: u64) -> usize {
    let mut c = 0;
    let mut t = n;

    while t > 0 {
        c += 1;
        t /= R;
    }

    c
}

pub fn split_digits<const R: u64>(n: u64, from_right: usize) -> (u64, u64) {
    let div = R.pow(from_right.try_into().unwrap());

    (n / div, n % div)
}

#[cfg(test)]
mod tests {
    use crate::util::{concat_numbers, count_digits, split_digits};

    #[test]
    fn test_concat() {
        assert_eq!(concat_numbers::<10>(12, 34), 1234);
        assert_eq!(concat_numbers::<10>(12, 0), 12);
    }

    #[test]
    fn test_count() {
        assert_eq!(count_digits::<10>(0), 0);
        assert_eq!(count_digits::<10>(1), 1);
        assert_eq!(count_digits::<10>(11), 2);
        assert_eq!(count_digits::<11>(10), 1);
    }

    #[test]
    fn test_split() {
        assert_eq!(split_digits::<10>(1234, 0), (1234, 0));
        assert_eq!(split_digits::<10>(1234, 1), (123, 4));
        assert_eq!(split_digits::<10>(1234, 2), (12, 34));
        assert_eq!(split_digits::<10>(1234, 3), (1, 234));
    }
}
