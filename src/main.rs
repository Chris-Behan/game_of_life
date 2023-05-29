use std::{thread, time};

fn main() {
    println!("Conway's game of life!");
    let mut grid = create_grid(20, 20);
    let initial_cords = vec![
        (3, 3),
        (4, 3),
        (5, 3),
        (14, 10),
        (15, 11),
        (12, 10),
        (13, 11),
        (13, 12),
    ];
    grid = set_initial_pattern(grid, initial_cords);
    let second = time::Duration::from_secs(1);
    let iterations = 100;
    print_grid(&grid);
    for _ in 0..iterations {
        thread::sleep(second);
        grid = simulate_life(&grid);
        print_grid(&grid);
    }
}

fn create_grid(width: usize, height: usize) -> Vec<Vec<bool>> {
    let grid: Vec<Vec<bool>> = vec![vec![false; width]; height];
    return grid;
}

fn print_grid(grid: &Vec<Vec<bool>>) {
    for row in grid {
        for alive in row {
            let tile = if *alive { '⬛' } else { ' ' };
            print!("{tile} ")
        }
        let space = " ".repeat(row.len());
        println!();
        println!("{space}");
    }
}

fn simulate_life(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_grid = grid.clone();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let alive = dead_or_alive(&grid, row, col);
            new_grid[row][col] = alive;
        }
    }
    return new_grid;
}

fn dead_or_alive(grid: &Vec<Vec<bool>>, row: usize, col: usize) -> bool {
    let alive = grid[row][col];
    let neighbours = number_of_neighbours(grid, row, col);
    if alive {
        if neighbours < 2 {
            return false;
        } else if neighbours == 2 || neighbours == 3 {
            return true;
        } else {
            return false;
        }
    }
    return if neighbours == 3 { true } else { false };
}

fn number_of_neighbours(grid: &Vec<Vec<bool>>, row: usize, col: usize) -> u64 {
    let mut count = 0;
    let row = i64::try_from(row).unwrap();
    let col = i64::try_from(col).unwrap();
    let neighbour_coordinates = vec![
        (row - 1, col - 1),
        (row - 1, col),
        (row - 1, col + 1),
        (row, col - 1),
        (row, col + 1),
        (row + 1, col - 1),
        (row + 1, col),
        (row + 1, col + 1),
    ];
    for (r, c) in neighbour_coordinates {
        if r < i64::try_from(grid.len()).unwrap()
            && r >= 0
            && c < i64::try_from(grid[0].len()).unwrap()
            && c >= 0
            && grid[r as usize][c as usize]
        {
            count += 1;
        }
    }

    return count;
}

fn set_initial_pattern(
    mut grid: Vec<Vec<bool>>,
    coordinates: Vec<(usize, usize)>,
) -> Vec<Vec<bool>> {
    for (row, col) in coordinates {
        grid[row][col] = true;
    }
    return grid;
}
