pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut result = 0;
    for line in lines {
        for c in line.chars() {
            if c.is_ascii_digit() {
                let num = c.to_digit(10).unwrap();
                result += num * 10;
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                let num = c.to_digit(10).unwrap();
                result += num;
                break;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let lines = input.lines();
    let mut result = 0;
    for line in lines {
        let mut firstnumber = 0;
        let mut firstindex = line.len();
        for (i, c) in line.chars().enumerate() {
            if let Some(num) = c.to_digit(10) {
                firstnumber = num;
                firstindex = i;
                break;
            }
        }
        if firstindex >= 3 {
            for (num, number_name) in map.iter().enumerate() {
                let i = line.find(number_name);
                if let Some(i) = i {
                    if i < firstindex {
                        firstindex = i;
                        firstnumber = (num + 1) as u32;
                    }
                }
            }
        }
        result += firstnumber * 10;

        //reverse ordering now

        let mut lastnumber = 0;
        let mut lastindex = 0;
        for (i, c) in line.chars().rev().enumerate() {
            if let Some(num) = c.to_digit(10) {
                lastnumber = num;
                lastindex = line.len() - 1 - i;
                break;
            }
        }
        if lastindex + 3 < line.len() {
            for (num, number_name) in map.iter().enumerate() {
                let i = line.rfind(number_name);
                if let Some(i) = i {
                    if i > lastindex {
                        lastindex = i;
                        lastnumber = (num + 1) as u32;
                    }
                }
            }
        }
        result += lastnumber;
    }

    Some(result)
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::submit::submit(1, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::submit::submit(1, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(209))
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(281));
    }
}
