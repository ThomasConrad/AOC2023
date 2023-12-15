fn parse(input: &str) -> impl Iterator<Item = [Vec<Vec<char>>; 2]> + '_ {
    input.split("\n\n").map(|line| {
        let grid: Vec<Vec<char>> = line.lines().map(|l| l.chars().collect()).collect();
        let mut grid_flipped = vec![vec!['.'; grid.len()]; grid[0].len()];
        for (i, row) in grid.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                grid_flipped[j][i] = *c;
            }
        }
        [grid, grid_flipped]
    })
}

fn count_diff(top: &Vec<char>, bottom: &Vec<char>) -> u32 {
    top.iter()
        .zip(bottom.iter())
        .map(|(a, b)| if a == b { 0 } else { 1 })
        .sum()
}

fn find_reflections(grids: [Vec<Vec<char>>; 2], goal_diff: u32) -> u32 {
    for (j, grid) in grids.iter().enumerate() {
        'outer: for i in 1..=grid.len() - 1 {
            let mut diffs = 0;
            let mut dist = 1;
            loop {
                //reach edge
                if i < dist || i + dist > grid.len() {
                    if diffs == goal_diff {
                        return if j == 1 { i } else { i * 100 } as u32;
                    }
                    continue 'outer;
                }
                diffs += count_diff(&grid[i - dist], &grid[i + dist - 1]);

                if diffs > goal_diff {
                    continue 'outer;
                }
                dist += 1;
            }
        }
    }
    panic!("No solution found");
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse(input).map(|s| find_reflections(s, 0)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parse(input).map(|s| find_reflections(s, 1)).sum())
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::submit::submit(13, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::submit::submit(13, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(405));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(400));
    }
}
