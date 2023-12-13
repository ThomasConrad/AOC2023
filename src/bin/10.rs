fn parse(input: &str) -> (Vec<Vec<char>>, [usize; 2]) {
    let mut map = Vec::new();
    let mut start = [0, 0];
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = [x, y];
            }
            row.push(c);
        }
        map.push(row);
    }
    (map, start)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapState {
    N,
    NW,
    NE,
    E,
    EN,
    ES,
    S,
    SE,
    SW,
    W,
    WN,
    WS,
    Inside,
    Outside,
}

fn to_char(value: &Option<MapState>) -> char {
    match value {
        Some(MapState::N) => '║',
        Some(MapState::NW) => '╗',
        Some(MapState::NE) => '╔',
        Some(MapState::E) => '═',
        Some(MapState::EN) => '╝',
        Some(MapState::ES) => '╗',
        Some(MapState::S) => '║',
        Some(MapState::SE) => '╚',
        Some(MapState::SW) => '╝',
        Some(MapState::W) => '═',
        Some(MapState::WN) => '╚',
        Some(MapState::WS) => '╔',
        Some(MapState::Inside) => 'I',
        Some(MapState::Outside) => 'O',
        None => ' ',
    }
}

fn to_dir(last_dir: &MapState, dir: &MapState) -> Option<MapState> {
    match (last_dir, dir) {
        (MapState::N, MapState::S) => Some(MapState::S),
        (MapState::S, MapState::N) => Some(MapState::N),
        (MapState::W, MapState::E) => Some(MapState::E),
        (MapState::E, MapState::W) => Some(MapState::W),
        (MapState::N, MapState::W) => Some(MapState::SW),
        (MapState::N, MapState::E) => Some(MapState::SE),
        (MapState::S, MapState::W) => Some(MapState::NW),
        (MapState::S, MapState::E) => Some(MapState::NE),
        (MapState::W, MapState::N) => Some(MapState::EN),
        (MapState::W, MapState::S) => Some(MapState::ES),
        (MapState::E, MapState::N) => Some(MapState::WN),
        (MapState::E, MapState::S) => Some(MapState::WS),
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, start) = parse(input);
    let mut pos;
    let mut dir;
    let mut steps = 0;

    //check directions
    if start[1] > 0
        && (map[start[1] - 1][start[0]] == '|'
            || map[start[1] - 1][start[0]] == '7'
            || map[start[1] - 1][start[0]] == 'F')
    {
        pos = [start[0], start[1] - 1];
        dir = MapState::N;
    } else if start[1] < map.len() - 1
        && (map[start[1] + 1][start[0]] == '|'
            || map[start[1] + 1][start[0]] == 'L'
            || map[start[1] + 1][start[0]] == 'J')
    {
        pos = [start[0], start[1] + 1];
        dir = MapState::S;
    } else if start[0] > 0
        && (map[start[1]][start[0] - 1] == '-'
            || map[start[1]][start[0] - 1] == 'F'
            || map[start[1]][start[0] - 1] == 'L')
    {
        pos = [start[0] - 1, start[1]];
        dir = MapState::W;
    } else if start[1] < map[0].len() - 1
        && (map[start[1]][start[0] + 1] == '-'
            || map[start[1]][start[0] + 1] == '7'
            || map[start[1]][start[0] + 1] == 'J')
    {
        pos = [start[0] + 1, start[1]];
        dir = MapState::E;
    } else {
        return None;
    }

    while pos != start {
        match dir {
            MapState::N => pos[1] -= 1,
            MapState::S => pos[1] += 1,
            MapState::W => pos[0] -= 1,
            MapState::E => pos[0] += 1,
            _ => panic!("Invalid direction"),
        }
        let c = map[pos[1]][pos[0]];
        dir = match (c, &dir) {
            ('|', MapState::S) => MapState::S,
            ('|', MapState::N) => MapState::N,
            ('-', MapState::W) => MapState::W,
            ('-', MapState::E) => MapState::E,
            ('L', MapState::W) => MapState::N,
            ('L', MapState::S) => MapState::E,
            ('J', MapState::S) => MapState::W,
            ('J', MapState::E) => MapState::N,
            ('7', MapState::N) => MapState::W,
            ('7', MapState::E) => MapState::S,
            ('F', MapState::N) => MapState::E,
            ('F', MapState::W) => MapState::S,
            ('S', _) => dir,
            _ => panic!("Invalid direction"),
        };

        steps += 1;
    }
    Some(steps / 2 + 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    //Start by replacing all unneded stuff and only have arrows and dots
    let (map, start) = parse(input);
    let mut new_map = vec![vec![None; map[0].len()]; map.len()];
    let mut pos = start;
    let mut dir = MapState::Outside;
    let mut last_dir = MapState::Outside;

    if start[1] > 0
        && (map[start[1] - 1][start[0]] == '|'
            || map[start[1] - 1][start[0]] == '7'
            || map[start[1] - 1][start[0]] == 'F')
    {
        dir = MapState::N;
    }
    if start[1] < map.len() - 1
        && (map[start[1] + 1][start[0]] == '|'
            || map[start[1] + 1][start[0]] == 'L'
            || map[start[1] + 1][start[0]] == 'J')
    {
        pos = [start[0], start[1] + 1];
        last_dir = dir;
        dir = MapState::S;
    }
    if start[0] > 0
        && (map[start[1]][start[0] - 1] == '-'
            || map[start[1]][start[0] - 1] == 'F'
            || map[start[1]][start[0] - 1] == 'L')
    {
        pos = [start[0] - 1, start[1]];
        last_dir = dir;
        dir = MapState::W;
    }
    if start[1] < map[0].len() - 1
        && (map[start[1]][start[0] + 1] == '-'
            || map[start[1]][start[0] + 1] == '7'
            || map[start[1]][start[0] + 1] == 'J')
    {
        pos = [start[0] + 1, start[1]];
        last_dir = dir;
        dir = MapState::E;
    }
    if dir == MapState::Outside || last_dir == MapState::Outside {
        panic!("Invalid start");
    }

    new_map[start[1]][start[0]] = to_dir(&last_dir, &dir);
    while pos != start {
        let c = map[pos[1]][pos[0]];
        last_dir = match &dir {
            MapState::N => MapState::S,
            MapState::S => MapState::N,
            MapState::W => MapState::E,
            MapState::E => MapState::W,
            _ => panic!("Invalid direction"),
        };
        dir = match (c, &dir) {
            ('|', MapState::S) => MapState::S,
            ('|', MapState::N) => MapState::N,
            ('-', MapState::W) => MapState::W,
            ('-', MapState::E) => MapState::E,
            ('L', MapState::W) => MapState::N,
            ('L', MapState::S) => MapState::E,
            ('J', MapState::S) => MapState::W,
            ('J', MapState::E) => MapState::N,
            ('7', MapState::N) => MapState::W,
            ('7', MapState::E) => MapState::S,
            ('F', MapState::N) => MapState::E,
            ('F', MapState::W) => MapState::S,
            ('S', _) => dir,
            _ => panic!("Invalid direction"),
        };
        new_map[pos[1]][pos[0]] = to_dir(&last_dir, &dir);
        match dir {
            MapState::N => pos[1] -= 1,
            MapState::S => pos[1] += 1,
            MapState::W => pos[0] -= 1,
            MapState::E => pos[0] += 1,
            _ => panic!("Invalid direction"),
        }
    }

    //Now the entire map is filled with arrows and dots
    let mut inside_amount = 0;
    for (y, line) in new_map.clone().into_iter().enumerate() {
        //we come from outside always
        let mut inside = false;
        let mut last_cross = 0; // 1 is up, -1 is down, 0 is none
        for (x, c) in line.iter().enumerate() {
            // println!("{} {}", y, x);
            if let Some(c) = c {
                match (last_cross, c) {
                    (1, &MapState::EN | &MapState::SW) => {
                        last_cross = 0;
                        inside = !inside;
                    }
                    (1, &MapState::ES | &MapState::NW) => {
                        last_cross = 0;
                    }
                    (-1, &MapState::EN | &MapState::SW) => {
                        last_cross = 0;
                    }
                    (-1, &MapState::ES | &MapState::NW) => {
                        last_cross = 0;
                        inside = !inside;
                    }
                    (0, &MapState::N | &MapState::S) => {
                        inside = !inside;
                    }
                    (0, &MapState::NE | &MapState::WS) => {
                        last_cross = 1;
                    }
                    (0, &MapState::SE | &MapState::WN) => {
                        last_cross = -1;
                    }
                    (_, &MapState::E | &MapState::W) => {}
                    _ => panic!("Wrong pipe state"),
                }
            } else if inside {
                inside_amount += 1;
                new_map[y][x] = Some(MapState::Inside);
            }
        }
    }
    #[cfg(debug_assertions)]
    for line in new_map.iter() {
        println!("{}", line.iter().map(to_char).collect::<String>());
    }
    Some(inside_amount)
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::submit::submit(10, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::submit::submit(10, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(23));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(4));
    }
}
