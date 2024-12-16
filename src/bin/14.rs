#[derive(Debug)]
struct RobotState {
    pos: [i32; 2],
    dir: [i32; 2],
}

fn parse(input: &str) -> Vec<RobotState> {
    let regex = regex::Regex::new(r"[-]?\d+").unwrap();
    input
        .lines()
        .map(|line| {
            let nums = regex
                .captures_iter(line)
                .map(|cap| cap[0].parse().unwrap())
                .collect::<Vec<i32>>();
            RobotState {
                pos: [nums[0], nums[1]],
                dir: [nums[2], nums[3]],
            }
        })
        .collect()
}

static WIDTH: i32 = 101;
static HEIGHT: i32 = 103;
static TIME: i32 = 100;

fn print_grid(states: &Vec<RobotState>) {
    let mut grid = vec![vec!['.'; WIDTH as usize]; HEIGHT as usize];

    for robot in states {
        grid[robot.pos[1] as usize][robot.pos[0] as usize] = '#';
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let states = parse(input);

    // println!("{:?}", states);

    let mut quads = [0, 0, 0, 0];

    for robot in states {
        let pos = [
            (robot.pos[0] + robot.dir[0] * TIME).rem_euclid(WIDTH),
            (robot.pos[1] + robot.dir[1] * TIME).rem_euclid(HEIGHT),
        ];

        // print_grid(pos);

        if pos[0] != WIDTH / 2 && pos[1] != HEIGHT / 2 {
            match (pos[0] > WIDTH / 2, pos[1] > HEIGHT / 2) {
                (true, true) => quads[0] += 1,
                (true, false) => quads[1] += 1,
                (false, true) => quads[2] += 1,
                (false, false) => quads[3] += 1,
            }
        }
    }

    // println!("{:?}", quads);

    Some(quads.iter().product())
}

fn count_overlap(states: &Vec<RobotState>) -> i32 {
    let mut grid = vec![vec![0; WIDTH as usize]; HEIGHT as usize];

    for robot in states {
        grid[robot.pos[1] as usize][robot.pos[0] as usize] += 1;
    }

    grid.iter().flatten().filter(|&&x| x > 1).count() as i32
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut states = parse(input);

    let mut i = 0;
    // if the robots coordinate, surely they won't collide. That is my test
    while count_overlap(&states) > 0 {
        for robot in states.iter_mut() {
            robot.pos[0] = (robot.pos[0] + robot.dir[0]).rem_euclid(WIDTH);
            robot.pos[1] = (robot.pos[1] + robot.dir[1]).rem_euclid(HEIGHT);
        }
        i += 1;
    }

    // print_grid(&states);

    Some(i as u32)
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
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), None);
    }
}
