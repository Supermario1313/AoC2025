use advent_of_code::iter_utils;
use ndarray::{Array, Array2, Axis};

advent_of_code::solution!(4);

// returns grid where grid[(i, j)] is true if cell (i, j) is a roll of paper
fn generate_grid(input: &str) -> Array2<bool> {
    let num_lines = input.chars().filter(|&c| c == '\n').count();
    let num_columns = input.chars().count() / num_lines - 1; // -1 because we don't want to count newlines

    let mut grid = Array::from_elem((num_lines, num_columns), false);
    for (i, line) in input.split('\n').enumerate() {
        if !line.is_empty() {
            for (j, c) in line.chars().enumerate() {
                if c == '@' {
                    grid[(i, j)] = true;
                }
            }
        }
    }

    grid
}

// Returns the number of neighboring rolls of `index` in `grid`
fn neighboring_rolls(grid: &Array2<bool>, index: (usize, usize)) -> u32 {
    let num_lines = grid.len_of(Axis(0));
    let num_columns = grid.len_of(Axis(1));

    let mut neighboring = 0;
    for neighbor_index in iter_utils::neighboring_indices(index, num_lines, num_columns) {
        if grid[neighbor_index] {
            neighboring += 1;
        }
    }

    neighboring
}

// Returns grid where grid[(i, j)] is Some(num_neighbors) if cell (i, j) is a roll of paper and None otherwise
fn generate_neighbor_grid(grid: &Array2<bool>) -> Array2<Option<u32>> {
    let num_lines = grid.len_of(Axis(0));
    let num_columns = grid.len_of(Axis(1));
    Array::from_shape_fn((num_lines, num_columns), |index| {
        match grid[index] {
            true => Some(neighboring_rolls(grid, index)),
            false => None
        }
    })
}

// Removes all accessible rolls from `grid`, update the number of neighbors and returns the amount of removed rolls
fn remove_accessible(grid: &mut Array2<Option<u32>>) -> u64 {
    let num_lines = grid.len_of(Axis(0));
    let num_columns = grid.len_of(Axis(1));

    // Coordinates of the removed rolls
    let mut removed_rolls: Vec<(usize, usize)> = vec![];

    // Remove all accessible rolls
    for (index, cell) in grid.indexed_iter_mut() {
        if let &mut Some(num_neighbors) = cell && num_neighbors < 4 {
            *cell = None;
            removed_rolls.push(index);
        }
    }

    // Decrease the number of neighbors of all rolls neigboring a removed roll
    for &index in removed_rolls.iter() {
        for neighbor_index in iter_utils::neighboring_indices(index, num_lines, num_columns) {
            if let Some(ref mut num_neighbors) = grid[neighbor_index] {
                *num_neighbors -= 1;
            }
        }
    }

    // Return the number of removed rolls
    removed_rolls.len() as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = generate_grid(input);

    let mut accessible_rolls = 0;

    for (index, &is_paper_roll) in grid.indexed_iter() {
        if is_paper_roll {
            if neighboring_rolls(&grid, index) < 4 {
                accessible_rolls += 1;
            }
        }
    }

    Some(accessible_rolls)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total_removed = 0;
    let mut grid = generate_neighbor_grid(&generate_grid(input));

    loop {
        let removed = remove_accessible(&mut grid);

        if removed == 0 {
            break;
        }

        total_removed += removed;
    }

    Some(total_removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
