use std::iter;

use itertools::Itertools;
use ndarray::{Array, Axis};

advent_of_code::solution!(6);

#[derive(Debug)]
enum Operation {
    Add,
    Mul
}

fn generate_operator_list(line: &str) -> Vec<Operation> {
    line.split_whitespace()
        .filter_map(|character| match character {
            "+" => Some(Operation::Add),
            "*" => Some(Operation::Mul),
            "" => None,
            _ => unreachable!()
        }).collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let num_lines = input.chars().filter(|&c| c == '\n').count();
    let mut line_iterator = input.split('\n').filter(|line| !line.is_empty());

    // Construct value grid
    let value_grid = {
        // List of all lines
        let lines = line_iterator.by_ref()
            .take(num_lines - 1)
            .map(|line| {
                Array::from_iter(line.split_whitespace().filter_map(|string| string.parse::<u64>().ok()))
            })
            .collect_vec();

        ndarray::stack(Axis(0), &lines.iter().map(|arr| arr.view()).collect_vec()).unwrap()
    };

    // Construct operator list
    let operator_list = generate_operator_list(line_iterator.next().unwrap());

    // Calculate total value
    let total_value = iter::zip(value_grid.axis_iter(Axis(1)), operator_list.iter())
        .map(|(values, operator)| {
            match operator {
                Operation::Add => values.sum(),
                Operation::Mul => values.product()
            }
        })
        .sum();

    Some(total_value)
}

pub fn part_two(input: &str) -> Option<u64> {
    let num_lines = input.chars().filter(|&c| c == '\n').count();
    let mut line_iterator = input.split('\n').filter(|line| !line.is_empty());

    // Construct character list
    let char_grid = {
        // List of all lines
        let lines = line_iterator.by_ref()
            .take(num_lines - 1)
            .map(|line| {
                Array::from_iter(line.chars())
            })
            .collect_vec();

        ndarray::stack(Axis(0), &lines.iter().map(|arr| arr.view()).collect_vec()).unwrap()
    };

    // Construct operator list
    let operator_list = generate_operator_list(line_iterator.next().unwrap());

    // Iterators
    let mut number_iterator = char_grid.axis_iter(Axis(1));

    // Calculate total value
    let total_value = operator_list.iter()
        .map(|operator| {
            let problem_value_iterator = number_iterator.by_ref()
                .take_while(|value| value.iter().any(|&char| char != ' '))
                .map(|value| value.iter().collect::<String>().trim().parse::<u64>().unwrap());

            match &operator {
                Operation::Add => problem_value_iterator.sum::<u64>(),
                Operation::Mul => problem_value_iterator.product::<u64>(),
            }
        })
        .sum();

    Some(total_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
