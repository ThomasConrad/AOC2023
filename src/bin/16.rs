use std::{cmp::Reverse, collections::HashSet};

use advent_of_code::helpers::get_safe;

fn parse(input: &str) -> (Vec<Vec<bool>>, [[isize; 2]; 2]) {
    let mut pos = [[0; 2]; 2];
    (
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '.' => false,
                        '#' => true,
                        'S' => {
                            pos[0] = [x as isize, y as isize];
                            false
                        }
                        'E' => {
                            pos[1] = [x as isize, y as isize];
                            false
                        }
                        _ => panic!("unexpected character"),
                    })
                    .collect()
            })
            .collect(),
        pos,
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn ccw(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }

    fn cw(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn to_index(&self) -> usize {
        match self {
            Self::Up => 0,
            Self::Right => 1,
            Self::Down => 2,
            Self::Left => 3,
        }
    }

    fn to_offset(&self) -> [isize; 2] {
        match self {
            Self::Up => [0, -1],
            Self::Right => [1, 0],
            Self::Down => [0, 1],
            Self::Left => [-1, 0],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    cost: u32,
    pos: [isize; 2],
    dir: Dir,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Returns a list of reachable states from the current state.
fn links(grid: &Vec<Vec<bool>>, state: &State) -> Vec<State> {
    let mut links = Vec::new();

    let [x, y] = state.pos;
    let [dx, dy] = state.dir.to_offset();

    let stepped_pos = [x + dx, y + dy];

    if !get_safe(stepped_pos, grid).unwrap_or(&true) {
        links.push(State {
            cost: state.cost + 1,
            pos: [x + dx, y + dy],
            dir: state.dir,
        });
    }

    for &new_dir in &[state.dir.ccw(), state.dir.cw()] {
        links.push(State {
            cost: state.cost + 1000,
            pos: [x, y],
            dir: new_dir,
        });
    }

    links
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, [start, end]) = parse(input);

    let mut cost = vec![vec![[None::<u32>; 4]; grid[0].len()]; grid.len()];

    let mut queue = std::collections::BinaryHeap::new();

    queue.push(State {
        cost: 0,
        pos: start,
        dir: Dir::Right,
    });

    while let Some(state) = queue.pop() {
        if state.pos == end {
            return Some(state.cost);
        }

        let [x, y] = state.pos;
        let idx = state.dir.to_index();

        if let Some(old_cost) = cost[y as usize][x as usize][idx] {
            if state.cost >= old_cost {
                continue;
            }
        }

        cost[y as usize][x as usize][idx] = Some(state.cost);

        for next_state in links(&grid, &state) {
            let [x, y] = next_state.pos;
            let idx = next_state.dir.to_index();
            if let Some(old_cost) = cost[y as usize][x as usize][idx] {
                if state.cost >= old_cost {
                    continue;
                }
            }

            queue.push(next_state);
        }
    }

    unreachable!()
}

#[derive(Debug, Clone)]
struct CostlyPath {
    cost: u32,
    path: HashSet<[isize; 2]>,
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, [start, end]) = parse(input);

    let mut cost = vec![vec![vec![None::<CostlyPath>; 4]; grid[0].len()]; grid.len()];
    cost[start[1] as usize][start[0] as usize][Dir::Right.to_index()] = Some(CostlyPath {
        cost: 0,
        path: [start].into_iter().collect(),
    });

    let mut queue = std::collections::BinaryHeap::new();

    queue.push(State {
        cost: 0,
        pos: start,
        dir: Dir::Right,
    });

    while let Some(state) = queue.pop() {
        if state.pos == end {
            return Some(
                cost[state.pos[1] as usize][state.pos[0] as usize][state.dir.to_index()]
                    .as_ref()
                    .unwrap()
                    .path
                    .len() as u32,
            );
        }
        let [x, y] = state.pos;
        let idx = state.dir.to_index();

        for next_state in links(&grid, &state) {
            let mut path = cost[y as usize][x as usize][idx].clone().unwrap().path;
            let [new_x, new_y] = next_state.pos;
            let new_idx = next_state.dir.to_index();
            path.insert([new_x, new_y]);

            match &mut cost[new_y as usize][new_x as usize][new_idx] {
                Some(current_best) => {
                    if next_state.cost > current_best.cost {
                        continue;
                    }

                    if next_state.cost == current_best.cost {
                        current_best.path.extend(path);
                    } else {
                        current_best.cost = next_state.cost;
                        current_best.path = path;
                    }
                }
                val @ None => {
                    *val = Some(CostlyPath {
                        cost: next_state.cost,
                        path,
                    });
                }
            }

            queue.push(next_state);
        }
    }
    unreachable!()
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
