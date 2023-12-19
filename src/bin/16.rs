use hashbrown::HashSet;

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c as u8).collect())
        .collect()
}

fn step(dir: i8, pos: (i32, i32)) -> (i32, i32) {
    match dir {
        0 => (pos.0 + 1, pos.1),
        1 => (pos.0, pos.1 - 1),
        2 => (pos.0 - 1, pos.1),
        3 => (pos.0, pos.1 + 1),
        _ => panic!("Invalid direction"),
    }
}

fn move_beam(
    dir: i8,
    pos: (i32, i32),
    grid: &Vec<Vec<u8>>,
    visited: &mut HashSet<(i8, (i32, i32))>,
) {
    let dir = dir.rem_euclid(4);
    let pos = step(dir, pos);
    // assert bounds
    if pos.0 < 0 || pos.1 < 0 || pos.0 >= grid.len() as i32 || pos.1 >= grid[0].len() as i32 {
        return;
    }
    if visited.contains(&(dir, pos)) {
        return;
    }
    visited.insert((dir, pos));
    //check tile

    match grid[pos.1 as usize][pos.0 as usize] {
        b'.' => {
            //continue straight
            // println!("continue straight");
            return move_beam(dir, pos, grid, visited);
        }
        b'/' => {
            //turn
            // println!("turn");
            if dir == 0 || dir == 2 {
                return move_beam(dir + 1, pos, grid, visited);
            }
            return move_beam(dir - 1, pos, grid, visited);
        }
        b'\\' => {
            //turn
            // println!("turn");
            if dir == 1 || dir == 3 {
                return move_beam(dir + 1, pos, grid, visited);
            }
            return move_beam(dir - 1, pos, grid, visited);
        }
        b'-' => {
            //continue
            if dir == 0 || dir == 2 {
                // println!("continue straight");
                return move_beam(dir, pos, grid, visited);
            }
            // println!("split");
            move_beam(dir + 1, pos, grid, visited);
            move_beam(dir - 1, pos, grid, visited);
            return;
        }
        b'|' => {
            //continue
            if dir == 1 || dir == 3 {
                // println!("continue straight");
                return move_beam(dir, pos, grid, visited);
            }

            // println!("split");
            move_beam(dir + 1, pos, grid, visited);
            move_beam(dir - 1, pos, grid, visited);
            return;
        }
        _ => panic!("Invalid input"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    let mut visited = HashSet::new();
    move_beam(0, (-1, 0), &input, &mut visited);

    Some(
        visited
            .iter()
            .map(|(_, pos)| pos)
            .collect::<std::collections::HashSet<_>>()
            .len() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    let border = [
        (
            0,
            (vec![-1; input[0].len()], (0..input.len() as i32).collect()),
        ),
        (
            1,
            (
                (0..input[0].len() as i32).collect(),
                vec![input.len() as i32; input.len()],
            ),
        ),
        (
            2,
            (
                vec![input[0].len() as i32; input[0].len()],
                (0..input.len() as i32).collect(),
            ),
        ),
        (
            3,
            ((0..input[0].len() as i32).collect(), vec![-1; input.len()]),
        ),
    ];
    let mut largest = 0;

    for (dir, (xs, ys)) in border {
        for (x, y) in xs.iter().zip(ys.iter()) {
            let mut visited = HashSet::new();
            move_beam(dir, (*x, *y), &input, &mut visited);
            largest = std::cmp::max(
                largest,
                visited
                    .iter()
                    .map(|(_, pos)| pos)
                    .collect::<std::collections::HashSet<_>>()
                    .len() as u32,
            )
        }
    }
    Some(largest)
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::submit::submit(16, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::submit::submit(16, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}
