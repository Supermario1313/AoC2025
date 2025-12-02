
/// Returns the number of digits of `n`.
pub fn num_digits(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

/// Splits `n` in half, returns `None` if its number of digits is even.
pub fn split_in_half(n: u64) -> Option<(u64, u64)> {
    let len = num_digits(n);

    if len.is_multiple_of(2) {
        let divisor = 10u64.pow(len / 2);
        Some((n / divisor, n % divisor))
    } else {
        None
    }
}

/// Splits `number` is `num_parts` parts, returns `None` if the number of digits of `number` is not a multiple of `num_parts`.
/// The returned list is little-endian.
pub fn split_in_parts(number: u64, num_parts: u32) -> Option<Vec<u64>> {
    let len = num_digits(number);
    let mut ret = vec![];

    if len.is_multiple_of(num_parts) {
        let divisor = 10u64.pow(len / num_parts);
        let mut dividend = number;

        for _ in 0 .. num_parts {
            ret.push(dividend % divisor);
            dividend = dividend / divisor;
        };

        Some(ret)

    } else {
        None
    }
}

/// Concats two integers together.
pub fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(num_digits(b)) + b
}

/// Concats a little-endian list of integers together, returns None if `list.len()` is `0`.
pub fn concat_list(list: &[u64]) -> Option<u64> {
    list.iter().copied().reduce(|acc, elem| concat(elem, acc))

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_digits() {
        assert_eq!(num_digits(0), 1);
        assert_eq!(num_digits(1), 1);
        assert_eq!(num_digits(10), 2);
        assert_eq!(num_digits(11), 2);
        assert_eq!(num_digits(123), 3);
        assert_eq!(num_digits(999), 3);
        assert_eq!(num_digits(1011), 4);
        assert_eq!(num_digits(1000000000), 10);
    }

    #[test]
    fn test_split_in_half() {
        assert_eq!(split_in_half(11), Some((1, 1)));
        assert_eq!(split_in_half(1011), Some((10, 11)));
        assert_eq!(split_in_half(123), None);
    }

    #[test]
    fn test_split_in_parts() {
        assert_eq!(split_in_parts(11, 2), Some(vec![1, 1]));
        assert_eq!(split_in_parts(1011, 2), Some(vec![11, 10]));
        assert_eq!(split_in_parts(123, 2), None);
        assert_eq!(split_in_parts(123, 3), Some(vec![3, 2, 1]));
        assert_eq!(split_in_parts(12345, 3), None);
        assert_eq!(split_in_parts(123456, 3), Some(vec![56, 34, 12]));
        assert_eq!(split_in_parts(123456, 2), Some(vec![456, 123]));
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat(12, 3), 123);
        assert_eq!(concat(1, 34), 134);
        assert_eq!(concat(123, 456), 123456);
        assert_eq!(concat(1200, 34), 120034);
    }

    #[test]
    fn test_concat_list() {
        assert_eq!(concat_list(&[3, 12]), Some(123));
        assert_eq!(concat_list(&[34, 1]), Some(134));
        assert_eq!(concat_list(&[456, 123]), Some(123456));
        assert_eq!(concat_list(&[34, 1200]), Some(120034));
        assert_eq!(concat_list(&[]), None);
        assert_eq!(concat_list(&[3, 2, 1]), Some(123));
        assert_eq!(concat_list(&[56, 34, 12]), Some(123456));
        assert_eq!(concat_list(&[456, 123]), Some(123456));
    }
}
