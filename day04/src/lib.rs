use std::cell::Cell;

use ndarray::{Array2, s};

pub fn read_grid(input: &str) -> Array2<u8> {
    let grid = input
        .lines()
        .map(|l| l.chars().map(|c| (c == '@') as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Array2::from_shape_vec(
        (grid.len(), grid[0].len()),
        grid.into_iter().flatten().collect(),
    )
    .unwrap()
}

pub fn padded_grid(grid: &Array2<u8>) -> Array2<u8> {
    let mut padded_grid = Array2::from_elem((grid.nrows() + 2, grid.ncols() + 2), 0u8);
    let (h, w) = padded_grid.dim();
    padded_grid.slice_mut(s![1..h - 1, 1..w - 1]).assign(grid);
    padded_grid
}

pub fn process_part1(input: &str) -> usize {
    let grid = read_grid(input);
    let padded_grid = padded_grid(&grid);

    padded_grid
        .windows((3, 3))
        .into_iter()
        .filter(|w| w[[1, 1]] == 1)
        .filter(|w| w.sum() < 5)
        .count()
}

pub fn process_part2(input: &str) -> usize {
    let grid = read_grid(input);
    let padded_grid = padded_grid(&grid).map(|x| Cell::new(*x));

    let mut total_count = 0;
    let mut count = grid.len();
    while count > 0 {
        count = padded_grid
            .windows((3, 3))
            .into_iter()
            .filter(|w| w[[1, 1]].get() == 1)
            .filter(|w| w.iter().map(|c| c.get()).sum::<u8>() < 5)
            .map(|w| w[[1, 1]].set(0))
            .count();
        total_count += count;
    }
    total_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            process_part1(
                "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@."
            ),
            13
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            process_part2(
                "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@."
            ),
            43
        );
    }
}
