use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(PartialEq)]
enum DiagramElem {
    Start,
    Empty,
    Splitter
}

fn generate_grid(input: &str) -> impl Iterator<Item = impl Iterator<Item = DiagramElem>> {
    input.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|char| match char {
            'S' => DiagramElem::Start,
            '.' => DiagramElem::Empty,
            '^' => DiagramElem::Splitter,
            _ => unreachable!()
        }))
}

pub fn part_one(input: &str) -> Option<u64> {
    // Diagram element iterator
    let mut grid = generate_grid(input);

    // beam_locations[i] == true if there is a beam at position i
    let mut beam_locations = grid.next().unwrap().map(|elem| elem == DiagramElem::Start).collect_vec();

    // How many splits we encountered
    let mut splits = 0;

    for line in grid {
        for (pos, element) in line.enumerate() {
            // The beam encounters a splitter
            if beam_locations[pos] && element == DiagramElem::Splitter {
                // Split the beam
                beam_locations[pos] = false;
                beam_locations[pos - 1] = true;
                beam_locations[pos + 1] = true;

                // Increase the split counter
                splits += 1;

            }
        }
    }

    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Diagram element iterator
    let mut grid = generate_grid(input);

    // superposed_beams[i] == j if there are j superposed beams at position i
    let mut superposed_beams = grid.next().unwrap().map(|elem| (elem == DiagramElem::Start) as u64).collect_vec();

    for line in grid {
        for (pos, element) in line.enumerate() {
            // We encounter a splitter
            if element == DiagramElem::Splitter {
                // Split the beams, does nothing if there are 0 superposed beams at position pos
                superposed_beams[pos - 1] += superposed_beams[pos];
                superposed_beams[pos + 1] += superposed_beams[pos];
                superposed_beams[pos] = 0;
            }
        }
    }

    // The timeline count is the number of beams we get at the end
    Some(superposed_beams.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
