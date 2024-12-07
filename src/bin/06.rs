use std::collections::HashSet;

use advent_of_code::helpers::get_safe;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn _turn_left(&mut self) {
        *self = match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }

    fn turn_right(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }

    fn step_back(&self, pos: &mut [isize; 2]) {
        match self {
            Self::Up => pos[1] += 1,
            Self::Down => pos[1] -= 1,
            Self::Left => pos[0] += 1,
            Self::Right => pos[0] -= 1,
        }
    }

    fn step(&self, pos: &mut [isize; 2]) {
        match self {
            Self::Up => pos[1] -= 1,
            Self::Down => pos[1] += 1,
            Self::Left => pos[0] -= 1,
            Self::Right => pos[0] += 1,
        }
    }
}

type Visited = bool;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty(Visited),
    Obstruction,
}

struct Map(Vec<Vec<Tile>>);

impl From<Vec<Vec<Tile>>> for Map {
    fn from(v: Vec<Vec<Tile>>) -> Self {
        Self(v)
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.0 {
            for tile in row {
                write!(
                    f,
                    "{}",
                    match tile {
                        Tile::Empty(true) => 'V',
                        Tile::Empty(false) => '.',
                        Tile::Obstruction => '#',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse(input: &str) -> (Map, [isize; 2], Dir) {
    let mut dir = Dir::Up;
    let mut pos = None;
    (
        input
            .lines()
            .enumerate()
            .map(|(col_num, line)| {
                line.chars()
                    .enumerate()
                    .map(|(row_num, c)| match c {
                        '.' => Tile::Empty(false),
                        '#' => Tile::Obstruction,
                        '^' => {
                            pos = Some([row_num as isize, col_num as isize]);
                            dir = Dir::Up;
                            Tile::Empty(true)
                        }
                        'V' => {
                            pos = Some([row_num as isize, col_num as isize]);
                            dir = Dir::Down;
                            Tile::Empty(true)
                        }
                        '<' => {
                            pos = Some([row_num as isize, col_num as isize]);
                            dir = Dir::Left;
                            Tile::Empty(true)
                        }
                        '>' => {
                            pos = Some([row_num as isize, col_num as isize]);
                            dir = Dir::Right;
                            Tile::Empty(true)
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect::<Vec<Vec<Tile>>>()
            .into(),
        pos.unwrap(),
        dir,
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut map, mut pos, mut dir) = parse(input);

    let mut visited_count = 1;

    //Start moving guard while he is "in the map"
    loop {
        dir.step(&mut pos);

        match get_safe(pos, &map.0) {
            Some(Tile::Empty(true)) => (),
            Some(Tile::Empty(false)) => {
                visited_count += 1;
                map.0[pos[1] as usize][pos[0] as usize] = Tile::Empty(true);
            }
            Some(Tile::Obstruction) => {
                dir.step_back(&mut pos);
                dir.turn_right();
            }
            None => break,
        }
    }
    Some(visited_count)
}

fn check_loop(map: &Map, mut pos: [isize; 2], mut dir: Dir, new_obstruction: [isize; 2]) -> bool {
    let mut seen = HashSet::new();
    loop {
        seen.insert((pos, dir));
        dir.step(&mut pos);

        if pos == new_obstruction {
            dir.step_back(&mut pos);
            dir.turn_right();
        } else {
            match get_safe(pos, &map.0) {
                Some(Tile::Obstruction) => {
                    dir.step_back(&mut pos);
                    dir.turn_right();
                }
                None => break,
                _ => (),
            }
        }
        if seen.contains(&(pos, dir)) {
            return true;
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut map, mut pos, mut dir) = parse(input);
    let starting_pos = pos;
    let starting_dir = dir;

    let mut loop_count = 0;
    //Start moving guard while he is "in the map"
    loop {
        dir.step(&mut pos);

        match get_safe(pos, &map.0) {
            Some(Tile::Empty(false)) => {
                if check_loop(&map, starting_pos, starting_dir, pos) {
                    loop_count += 1;
                    map.0[pos[1] as usize][pos[0] as usize] = Tile::Empty(true);
                }
            }
            Some(Tile::Obstruction) => {
                dir.step_back(&mut pos);
                dir.turn_right();
            }
            None => break,
            _ => (),
        }
    }
    Some(loop_count)
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::submit::submit(6, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::submit::submit(6, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(41));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
