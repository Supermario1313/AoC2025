use advent_of_code::int_utils::{self};
use itertools::Itertools;

advent_of_code::solution!(3);

fn iter_banks(input: &str) -> impl Iterator<Item = Vec<u64>> {
    input.split('\n')
        .filter_map(|line| {
            if !line.is_empty() {
                Some(line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect_vec())
            } else {
                None
            }
        })
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut acc = 0u64;
    for bank in iter_banks(input) {

        // The most significant digit is the largest digit in range 0 .. len-1
        let mut first_digit_index = 0usize;
        for i in 1 .. bank.len() - 1 {
            if bank[i] > bank[first_digit_index] {
                first_digit_index = i;
            }
        }

        // The least significant digit is the largest digit in range first_digit_index .. len
        let mut second_digit = bank[first_digit_index + 1];
        for i in first_digit_index + 2 .. bank.len() {
            if bank[i] > second_digit {
                second_digit = bank[i];
            }
        }

        // Add rating to accumulator
        acc += bank[first_digit_index] * 10 + second_digit;
    }

    Some(acc)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut acc = 0u64;
    for bank in iter_banks(input) {
        let mut digits: Vec<u64> = vec![];

        // The nth digit (where the 0th digit is the least significant digit) is the largest in range previous_digit_index + 1 .. len - n
        let mut current_index = 0usize;
        for digit_num in (0 .. 12).rev() {
            let mut digit_index = current_index;

            for i in current_index + 1 .. bank.len() - digit_num {
                if bank[i] > bank[digit_index] {
                    digit_index = i;
                }
            }

            digits.push(bank[digit_index]);
            current_index = digit_index + 1;
        }

        // Calculate and add rating to the accumulator
        acc += int_utils::horner(&digits, 10);
    }

    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
