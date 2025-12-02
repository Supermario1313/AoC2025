use iter_accumulate::IterAccumulate;

advent_of_code::solution!(1);

fn iterate_rotations(input: &str) -> impl Iterator<Item = i32> {
    input.split("\n").filter_map(|rotation| {
        if !rotation.is_empty() {
            let rotation_amount: i32 = rotation[1..].parse().unwrap();

            match rotation.chars().nth(0) {
                Some('L') => Some(-rotation_amount),
                Some('R') => Some(rotation_amount),
                _ => unreachable!()
            }
        } else {
            None
        }
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut num_zeros = 0u64;

    for position in iterate_rotations(input).accumulate(50, |acc, rot| (acc + rot).rem_euclid(100)) {
        if position == 0 {
            num_zeros += 1;
        }
    }

    Some(num_zeros)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut num_zeros = 0u64;
    let mut cur_position = 50;

    for rotation in iterate_rotations(input) {
        let num_turns = (rotation / 100).abs() as u64;
        let remaining_steps = rotation % 100;

        num_zeros += num_turns;

        let next_position = (cur_position + remaining_steps).rem_euclid(100);

        if (remaining_steps > 0 && next_position <= cur_position)
        || (remaining_steps < 0 && cur_position != 0 && (next_position == 0 || next_position >= cur_position)) {
            num_zeros += 1;
        }

        cur_position = next_position;
    }

    Some(num_zeros)
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
        assert_eq!(result, Some(6));
    }
}
