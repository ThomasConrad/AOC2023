use hashbrown::HashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    dist: u32,
    dir: i8,
    pos: [i32; 2],
    consecutive: i8,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .dist
            .cmp(&self.dist)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn step(dir: i8, pos: [i32; 2]) -> [i32; 2] {
    match dir {
        0 => [pos[0] + 1, pos[1]],
        1 => [pos[0], pos[1] - 1],
        2 => [pos[0] - 1, pos[1]],
        3 => [pos[0], pos[1] + 1],
        _ => panic!("Invalid direction"),
    }
}

fn get_nbrs(state: State, grid: &Vec<Vec<u32>>) -> Vec<State> {
    let mut nbrs: Vec<(i8, [i32; 2], i8)> = [-1, 1]
        .iter()
        .map(|&d| (state.dir + d).rem_euclid(4))
        .map(|d| (d, step(d, state.pos), 1))
        .collect();
    if state.consecutive < 3 {
        nbrs.push((state.dir, step(state.dir, state.pos), state.consecutive + 1));
    }
    nbrs.iter()
        .filter(|(_, p, _)| {
            p[0] >= 0 && p[0] < grid[0].len() as i32 && p[1] >= 0 && p[1] < grid.len() as i32
        })
        .map(|&(d, p, c)| State {
            dist: state.dist + grid[p[1] as usize][p[0] as usize],
            dir: d,
            pos: p,
            consecutive: c,
        })
        .collect()
}

fn get_ultra_nbrs(state: State, grid: &Vec<Vec<u32>>) -> Vec<State> {
    let mut nbrs = Vec::new();
    if state.consecutive < 10 {
        nbrs.push((state.dir, step(state.dir, state.pos), state.consecutive + 1));
    }
    if state.consecutive >= 4 {
        nbrs.extend(
            [-1, 1]
                .iter()
                .map(|&d| (state.dir + d).rem_euclid(4))
                .map(|d| (d, step(d, state.pos), 1)),
        );
    }

    nbrs.iter()
        .filter(|(_, p, _)| {
            p[0] >= 0 && p[0] < grid[0].len() as i32 && p[1] >= 0 && p[1] < grid.len() as i32
        })
        .map(|&(d, p, c)| State {
            dist: state.dist + grid[p[1] as usize][p[0] as usize],
            dir: d,
            pos: p,
            consecutive: c,
        })
        .collect()
}

fn dijkstra(grid: &Vec<Vec<u32>>, start: [i32; 2], end: [i32; 2], ultra: bool) -> Option<u32> {
    let mut visited = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(State {
        dist: 0,
        dir: 0,
        pos: start,
        consecutive: 0,
    });
    while let Some(state) = queue.pop() {
        if state.pos == end && (!ultra || state.consecutive >= 4) {
            return Some(state.dist);
        }
        if visited.contains_key(&(state.pos, state.dir, state.consecutive)) {
            continue;
        }
        visited.insert((state.pos, state.dir, state.consecutive), state.dist);

        queue.extend(if ultra {
            get_ultra_nbrs(state, grid)
        } else {
            get_nbrs(state, grid)
        });
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    let start = [0, 0];
    let end = [input[0].len() as i32 - 1, input.len() as i32 - 1];

    dijkstra(&input, start, end, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    let start = [0, 0];
    let end = [input[0].len() as i32 - 1, input.len() as i32 - 1];

    dijkstra(&input, start, end, true)
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::submit::submit(17, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::submit::submit(17, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(102));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(94));
    }
}
