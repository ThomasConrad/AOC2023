use hashbrown::HashMap;

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|c| c.as_bytes().to_vec()).collect()
}

fn tilt_north(grid: &mut Vec<Vec<u8>>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == b'O' {
                //scan north
                let mut i = 1;
                while i <= y {
                    if grid[y - i][x] != b'.' {
                        break;
                    }
                    i += 1;
                }
                if i > 1 {
                    grid[y + 1 - i][x] = b'O';
                    grid[y][x] = b'.';
                }
            }
        }
    }
}

fn rotate_cw(grid: &mut Vec<Vec<u8>>) {
    let mut new_grid = vec![vec![b'.'; grid.len()]; grid[0].len()];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            new_grid[y][x] = grid[grid.len() - 1 - x][y];
        }
    }
    *grid = new_grid;
}

fn inplace_rotate_cw(grid: &mut Vec<Vec<u8>>) {
    assert_eq!(grid.len(), grid[0].len());
    let n = grid.len();
    for i in 0..n / 2 {
        for j in i..(n - i - 1) {
            let tmp = grid[i][j];
            grid[i][j] = grid[n - 1 - j][i];
            grid[n - 1 - j][i] = grid[n - 1 - i][n - 1 - j];
            grid[n - 1 - i][n - 1 - j] = grid[j][n - 1 - i];
            grid[j][n - 1 - i] = tmp;
        }
    }
}

fn grid_hash(grid: &Vec<Vec<u8>>) -> u32 {
    let mut hash = 1u32;
    for line in grid.iter() {
        for &c in line.iter() {
            hash = hash.wrapping_mul(31).wrapping_add(c as u32);
        }
    }
    hash
}

fn grid_load(grid: &Vec<Vec<u8>>) -> u32 {
    grid.iter()
        .rev()
        .enumerate()
        .map(|(load_lvl, line)| {
            line.iter().filter(|&&c| c == b'O').count() as u32 * (load_lvl as u32 + 1)
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = parse(input);
    tilt_north(&mut grid);
    Some(grid_load(&grid))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = parse(input);
    let mut loop_found = false;
    let mut seen_states = HashMap::new();
    let mut i = 0u32;
    while i < 1_000_000_000 {
        for _ in 0..4 {
            tilt_north(&mut grid);
            inplace_rotate_cw(&mut grid);
        }
        if !loop_found {
            if let Some(cycle_start) = seen_states.get(&grid_hash(&grid)) {
                let cycle_len = i - cycle_start;
                let repeats = (1_000_000_000 - i) / cycle_len;
                i += repeats * cycle_len;
                loop_found = true;
            }
            seen_states.insert(grid_hash(&grid), i);
        }
        i += 1;
    }

    Some(grid_load(&grid))
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::submit::submit(14, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::submit::submit(14, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(136));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(64));
    }
}
