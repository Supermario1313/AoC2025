use itertools::Itertools;

advent_of_code::solution!(5);

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut line_iter = input.split('\n');

    // Parse ranfes
    let ranges = line_iter.by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.split('-')
                .map(|string| string.parse().unwrap())
                .next_tuple()
                .unwrap()
        }).collect_vec();

    // Parse ingredients
    let ingredients = line_iter.take_while(|line| !line.is_empty()).map(|line| line.parse().unwrap()).collect_vec();

    (ranges, ingredients)
}

fn sort_and_merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)>{
    if ranges.is_empty() {
        ranges
    } else {
        // Merged and sorted ranges
        let mut merged_sorted_ranges = vec![];

        // Sort ranges by first element
        ranges.sort_unstable_by_key(|&(a, _)| a);

        // Convert ranges to iterator
        let mut range_iter = ranges.into_iter();

        // First range in list
        let mut current_range = range_iter.next().unwrap();

        // Iter and merge remaining ranges
        for range in range_iter {
            if range.0 <= current_range.1 + 1 {
                // Merge ranges if they can be merged
                current_range.1 = current_range.1.max(range.1);
            } else {
                // Push current range and replace it
                merged_sorted_ranges.push(current_range);
                current_range = range;
            }
        }

        // Push last range
        merged_sorted_ranges.push(current_range);

        merged_sorted_ranges
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut ranges, mut ingredients) = {
        let (ranges_vec, ingredients_vec) = parse_input(input);

        (sort_and_merge_ranges(ranges_vec).into_iter(), ingredients_vec.into_iter().sorted())
    };

    // Fresh ingredient counter
    let mut fresh_ingredients = 0;

    // The range we're currently checking whether the ingredients belong to or not, None if there are no remaining ranges
    let mut current_range = ranges.next();

    // The ingredient we're currently checking
    let mut current_ingredient = ingredients.next();

    // Iter until we've processed all ingredients or there are no remaining ranges (the remaining ingredients are beyond all ranges and thus not fresh)
    while let Some(ingredient) = current_ingredient && let Some((first, last)) = current_range {
        if ingredient < first {
            // The ingredient is before the range, meaning it's not fresh. Get the next ingredient without incrementing the fresh ingredient counter
            current_ingredient = ingredients.next();
        } else if ingredient > last {
            // The ingredient is beyond the sample, try with the next range
            current_range = ranges.next();
        } else {
            // The ingredient is within the range, increment the fresh ingredient counter and get the next ingredient
            fresh_ingredients += 1;
            current_ingredient = ingredients.next();
        }
    }

    Some(fresh_ingredients)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Fresh ingredient counter
    let fresh_ingredients = sort_and_merge_ranges(parse_input(input).0).into_iter()
        .fold(0, |acc, (first, last)| acc + last - first + 1);

    Some(fresh_ingredients)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
