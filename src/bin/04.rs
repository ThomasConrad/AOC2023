use advent_of_code::helpers::get_safe;

static DIRECTIONS: [[isize; 2]; 8] = [
    [0, 1],
    [1, 0],
    [0, -1],
    [-1, 0],
    [1, 1],
    [-1, -1],
    [1, -1],
    [-1, 1],
];

fn start_search(field: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    if field[y as usize][x as usize] != 'X' {
        return 0;
    }
    let mut count = 0;

    'outer: for dir in DIRECTIONS {
        let mut x = x as isize;
        let mut y = y as isize;

        for letter in ['M', 'A', 'S'] {
            x += dir[0];
            y += dir[1];
            if get_safe([x, y], field) != Some(&letter) {
                continue 'outer;
            }
        }
        count += 1;
    }
    count
}

fn search_cross(field: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if field[y as usize][x as usize] != 'A' {
        return false;
    }

    for diag in [[1, 1], [1, -1]] {
        let diag_fwd = get_safe([x as isize + diag[0], y as isize + diag[1]], field);
        let diag_bwd = get_safe([x as isize - diag[0], y as isize - diag[1]], field);
        match (diag_fwd, diag_bwd) {
            (Some('M'), Some('S')) | (Some('S'), Some('M')) => (),
            _ => return false,
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let field = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut count = 0;
    for y in 0..field.len() {
        for x in 0..field[y].len() {
            count += start_search(&field, x as usize, y as usize);
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let field = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut count = 0;
    for y in 1..field.len() - 1 {
        for x in 1..field[y].len() - 1 {
            if search_cross(&field, x as usize, y as usize) {
                count += 1;
            }
        }
    }
    Some(count)
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::submit::submit(4, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::submit::submit(4, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(9));
    }
}
