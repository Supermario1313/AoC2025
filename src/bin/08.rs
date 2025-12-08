use std::{cmp::Ordering, sync::LazyLock};

use advent_of_code::int_utils;
use itertools::{Itertools, iproduct};
use regex::Regex;
use partial_sort::PartialSort;
use disjoint::DisjointSet;

advent_of_code::solution!(8);

static COORD_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d+),(\d+),(\d+)").unwrap());

fn parse_coords(input: &str) -> Vec<(u64, u64, u64)> {
    COORD_RE.captures_iter(input)
        .map(|capture| {
            let (_, [x, y, z]) = capture.extract();
            (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
        }).collect_vec()
}

fn distance_ordering(boxes: &[(u64, u64, u64)]) -> impl Fn(&(usize, usize), &(usize, usize)) -> Ordering {
    |&(i, j), &(k, l)| int_utils::squared_eucl_3d(boxes[i], boxes[j]).cmp(&int_utils::squared_eucl_3d(boxes[k], boxes[l]))
}

fn part_one_aux(input: &str, num_pairs: usize) -> Option<u64> {
    // Junction box list
    let boxes = parse_coords(input);

    // Box count
    let num_boxes = boxes.len();

    // Box index pairs ordered by distance
    let mut box_pairs = iproduct!(0..num_boxes, 0..num_boxes).filter(|&(i, j)| i < j).collect_vec();
    box_pairs.partial_sort(num_pairs, distance_ordering(&boxes));

    // Union-find data structure representing circuits
    let mut circuits = DisjointSet::with_len(num_boxes);

    // Link the closest box pairs together
    for (i, j) in box_pairs.into_iter().take(num_pairs) {
        circuits.join(i, j);
    }

    // Vec of circuit lengths
    let mut circuit_lengths = circuits.sets().into_iter().map(|circuit| circuit.len() as u64).collect_vec();

    // Find 3 largest circuits
    circuit_lengths.partial_sort(3, |&x, &y| x.cmp(&y).reverse());

    // Return product of 3 largest circuits
    Some(circuit_lengths.into_iter().take(3).product())
}

pub fn part_one(input: &str) -> Option<u64> {
    part_one_aux(input, 1000)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Junction box list
    let boxes = parse_coords(input);

    // Box count
    let num_boxes = boxes.len();

    // Box index pairs ordered by distance
    let mut box_pairs = iproduct!(0..num_boxes, 0..num_boxes).filter(|&(i, j)| i < j).collect_vec();
    box_pairs.sort_by(distance_ordering(&boxes));

    // Last pair of boxes joined together
    let mut last_junction = None;

    // Union-find data structure representing circuits
    let mut circuits = DisjointSet::with_len(num_boxes);

    // Link the closest box pairs together
    for (i, j) in box_pairs.into_iter() {
        if circuits.root_of(i) != circuits.root_of(j) {
            circuits.join(i, j);
            last_junction = Some((i, j))
        }
    }

    // Multiply x coordinates of last joined boxes
    last_junction.map(|(i, j)| boxes[i].0 * boxes[j].0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_aux(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
