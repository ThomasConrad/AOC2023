fn parse(input: &str) -> Vec<(i64, i64, i64, i64)> {
    input
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            let dir = get_dir(words.next().unwrap().as_bytes()[0]);
            let len = words.next().unwrap().parse().unwrap();
            let hex_string = words.next().unwrap().get(2..8).unwrap();
            let len_long = i64::from_str_radix(&hex_string[0..5], 16).unwrap();
            let dir_long = match hex_string.chars().nth(5).unwrap().to_digit(10).unwrap() {
                0 => 0,
                1 => 3,
                2 => 2,
                3 => 1,
                _ => panic!("Invalid direction"),
            };
            (dir, len, dir_long, len_long)
        })
        .collect()
}

fn get_dir(point: u8) -> i64 {
    match point {
        b'R' => 0,
        b'U' => 1,
        b'L' => 2,
        b'D' => 3,
        _ => panic!("Invalid direction"),
    }
}

fn solve_area(input: Vec<(i64, i64)>) -> u64 {
    let mut verts = vec![[0, 0]];
    let mut pos = [1, 0];
    for i in 0..input.len() {
        let dir = input[i].0;
        let s1 = dir - input[(i + 1) % input.len()].0;
        let s = if s1.abs() == 1 { s1 } else { -s1.signum() };
        pos[dir as usize % 2] += (if dir > 1 { -1 } else { 1 }) * input[i].1;
        if s == -1 {
            pos[dir as usize % 2] -= if dir > 1 { -1 } else { 1 };
        }
        verts.push(pos.clone());

        if s == 1 {
            pos[(dir + 1) as usize % 2] += if (dir - s).rem_euclid(4) > 1 { -1 } else { 1 };
        }
    }
    let len = verts.len();
    //Shoelace formula
    let mut area = 0;
    for i in 0..len {
        area += verts[i][0] * verts[(i + 1) % len][1];
        area -= verts[i][1] * verts[(i + 1) % len][0];
    }
    (area.abs() / 2) as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve_area(
        parse(input).iter().map(|x| (x.0, x.1)).collect(),
    ))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve_area(
        parse(input).iter().map(|x| (x.2, x.3)).collect(),
    ))
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::submit::submit(18, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::submit::submit(18, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(62));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(952408144115));
    }
}
