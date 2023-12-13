fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solve(input: &str, dilation: u64) -> Option<u64> {
    let grid = parse(input);
    let empty_rows = grid
        .iter()
        .enumerate()
        .filter_map(|(y, line)| {
            if line.iter().all(|&c| c == '.') {
                Some(y)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let empty_columns = (0..grid[0].len())
        .filter(|&x| grid.iter().all(|line| line[x] == '.'))
        .collect::<Vec<_>>();

    //find # locations
    let mut locations = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '#' {
                let xcount =
                    empty_columns.iter().filter(|&&xx| xx <= x).count() as u64 * (dilation - 1);
                let ycount =
                    empty_rows.iter().filter(|&&yy| yy <= y).count() as u64 * (dilation - 1);
                locations.push(((x as u64 + xcount) as i64, (y as u64 + ycount) as i64));
            }
        }
    }

    //Generate all pairs
    let mut pairs = Vec::new();
    for i in 0..locations.len() {
        for j in i + 1..locations.len() {
            pairs.push((locations[i], locations[j]));
        }
    }

    //Calculate path lengths
    Some(
        pairs
            .iter()
            .map(|&(a, b)| (a.0 - b.0).abs() + (a.1 - b.1).abs())
            .sum::<i64>() as u64,
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 1000000)
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::submit::submit(11, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::submit::submit(11, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(374));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(82000210));
    }
}
