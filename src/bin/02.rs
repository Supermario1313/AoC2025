use core::num;
use std::{collections::HashSet, sync::LazyLock};

use advent_of_code::int_utils;
use iter_peek_end::IterPeekEnd;
use itertools::Itertools;
use primes::{PrimeSet, TrialDivision};
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
fn previous_invalid_id(id: u64, num_parts: u32) -> Option<u64> {
    match int_utils::split_in_parts(id, num_parts) {
        // Number of digits is a multiple of num_parts, calculate the previous id
        Some(parts) => {
            let [.., head] = parts[..] else {
                unreachable!();
            };

            if id >= int_utils::repeat(head, num_parts).unwrap() {
                // id >= CONCAT(head, ..., head), therefore the previous invalid id is CONCAT(head, ..., head)
                Some(head)
            } else {
                // id < CONCAT(head, ..., head), therefore the previous invalid id is CONCAT(head-1, ..., head-1)
                Some(head - 1)
            }
        },

        // Number of digits is not a multiple of num_parts, the previous invalid id is the largest invalid id with
        // a number of digits equals to the previous multiple of num_parts
        None => {
            let num_digits = int_utils::num_digits(id);

            if num_digits >= num_parts {
                Some(10u64.pow(int_utils::num_digits(id) / num_parts) - 1)
            } else {
                None
            }
        }
    }
}

/// Returns the smallest integer n such that CONCAT(n,n) is an invalid id greater than or equal to the argument
fn next_invalid_id(id: u64, num_parts: u32) -> u64 {
    match int_utils::split_in_parts(id, num_parts) {
        // Number of digits is a multiple of num_parts, calculate the previous id
        Some(parts) => {
            let [.., head] = parts[..] else {
                unreachable!();
            };

            if id <= int_utils::repeat(head, num_parts).unwrap() {
                // id <= CONCAT(head, ..., head), therefore the next invalid id is CONCAT(head, ..., head)
                head
            } else {
                // id > CONCAT(head, ..., head), therefore the next invalid id is CONCAT(head+1, ..., head+1)
                head + 1
            }
        },

        // Number of digits is not a multiple of num_parts, the next invalid id is the smallest invalid id with
        // a number of digits equals to the next multiple of num_parts
        None => 10u64.pow(int_utils::num_digits(id) / num_parts)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut acc = 0u64;
    for (from, to) in iter_ranges(input) {
        if let Some(max_invalid) = previous_invalid_id(to, 2) {
            let min_invalid = next_invalid_id(from, 2);

            for invalid in min_invalid ..= max_invalid {
                acc += int_utils::concat(invalid, invalid);
            }
        }
    }

    Some(acc)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Prime number generator
    let mut prime_generator = TrialDivision::new();

    // The sum of all invalid IDs
    let mut acc = 0u64;

    // Iterate through all ID ranges
    for (from, to) in iter_ranges(input) {
        // The max length of an ID in this range
        let max_length = int_utils::num_digits(from).max(int_utils::num_digits(to));

        // Set of already seen invalid IDs to avoid duplicates
        let mut seen_invalid_ids: HashSet<u64> = HashSet::new();

        // All possible ways to subdivide a number whose length is at most max_length
        // Note that we only care about dividing numbers into a prime number of parts since, if an invalid id is made up of a pattern
        // repeating n times, it can be considered as a pattern repeating d times for every divisor d of n
        let mut possible_subdivisions = prime_generator.iter()
            .map_while(|length| if length <= max_length as u64 {Some(length as u32)} else {None})
            .collect_vec().into_iter().rev() // There are less invalid ids with a large subdivision, so we handle them first to avoid filling seen_invalid_ids
            .peekable(); // To know when we reach the last iteration

        // Iterate all possible subdivisions
        while let Some(num_parts) = possible_subdivisions.next() {
            // Check if there are invalid ids smaller than to and get the larger one
            if let Some(max_invalid_fragment) = previous_invalid_id(to, num_parts) {
                // Get the smallest invalid id larger than from
                let min_invalid_fragment = next_invalid_id(from, num_parts);

                // Iterate through all invalid ids between from and to
                for invalid_fragment in min_invalid_fragment ..= max_invalid_fragment {
                    // Construct the invalid ID
                    let invalid_id = int_utils::repeat(invalid_fragment, num_parts).unwrap();

                    // Check if we have encountered this ID
                    if !seen_invalid_ids.contains(&invalid_id) {
                        // If we haven't, add it to the accumulator
                        acc += invalid_id;

                        // Add this ID to the set of seen IDs, unless we're at the last iteration
                        if possible_subdivisions.is_not_last() {
                            seen_invalid_ids.insert(invalid_id);
                        }
                    }
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
