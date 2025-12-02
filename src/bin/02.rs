use std::sync::LazyLock;

use advent_of_code::int_utils;
use regex::Regex;

advent_of_code::solution!(2);

static RANGE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d+)-(\d+)").unwrap());

fn iter_ranges(input: &str) -> impl Iterator<Item = (u64, u64)> {
    RANGE_RE.captures_iter(input).map(|capture| {
        let (_, [from, to]) = capture.extract();
        (from.parse().unwrap(), to.parse().unwrap())
    })
}

/// Returns the largent integer n such that CONCAT(n,n) is an invalid id smaller than or equal to the argument
fn previous_invalid_id(id: u64) -> Option<u64> {
    match int_utils::split_in_half(id) {
        // Number of digits is even, calculate the previous id
        Some((head, tail)) => {
            if tail >= head {
                // tail >= head, therefore the previous invalid id is CONCAT(head,head)
                Some(head)
            } else {
                // tail < head, therefore the previous invalid id is CONCAT(head-1,head-1)
                Some(head - 1)
            }
        },

        // Number of digits is odd, the next invalid id is the smallest invalid id with num_digits - 1 digits
        None => {
            let num_digits = int_utils::num_digits(id);

            if num_digits > 1 {
                Some(10u64.pow(int_utils::num_digits(id) / 2) - 1)
            } else {
                None
            }
        }
    }
}

/// Returns the smallest integer n such that CONCAT(n,n) is an invalid id greater than or equal to the argument
fn next_invalid_id(id: u64) -> u64 {
    match int_utils::split_in_half(id) {
        // Number of digits is even, calculate the next id
        Some((head, tail)) => {
            if tail <= head {
                // tail <= head, therefore the next invalid id is CONCAT(head,head)
                head
            } else {
                // tail > head, therefore the next invalid id is CONCAT(head+1,head+1)
                head + 1
            }
        },

        // Number of digits is odd, the next invalid id is the smallest invalid id with num_digits + 1 digits
        None => 10u64.pow(int_utils::num_digits(id) / 2)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut acc = 0u64;
    for (from, to) in iter_ranges(input) {
        if let Some(max_invalid) = previous_invalid_id(to) {
            let min_invalid = next_invalid_id(from);

            for invalid in min_invalid ..= max_invalid {
                acc += int_utils::concat(invalid, invalid);
            }
        }
    }

    Some(acc)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut acc = 0u64;
    for (from, to) in iter_ranges(input) {
        let max_length = int_utils::num_digits(from).max(int_utils::num_digits(to));
        for num_parts in 2 ..= max_length {
            if let Some(max_invalid) = previous_invalid_id(to) {
                let min_invalid = next_invalid_id(from);

                for invalid in min_invalid ..= max_invalid {
                    acc += int_utils::concat(invalid, invalid);
                }
            }
        }
    }

    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
