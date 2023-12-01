use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut result = 0;
    for line in lines {
        for c in line.chars() {
            if c.is_digit(10) {
                let num = c.to_digit(10).unwrap();
                result += num * 10;
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_digit(10) {
                let num = c.to_digit(10).unwrap();
                result += num;
                break;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let lines = input.lines();
    let mut result = 0;
    for (idx, line) in lines.enumerate() {
        let mut firstnumber = 0;
        let mut firstindex = line.len();
        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                let num = c.to_digit(10).unwrap();
                firstnumber = num;
                firstindex = i;
                break;
            }
        }
        for number_name in map.keys() {
            // println!("number_name: {}", number_name);
            // println!("line: {}", line);
            let mut matches: Vec<usize> = line.match_indices(number_name).map(|m| m.0).collect();
            matches.sort();
            if matches.len() > 0 {
                let i = matches.first().unwrap();
                if i < &firstindex {
                    firstindex = *i;
                    firstnumber = *map.get(number_name).unwrap();
                }
            }
        }
        assert!(firstindex < line.len());
        assert!(firstnumber > 0);
        // println!("firstnumber: {}", firstnumber);
        result += firstnumber * 10;

        //reverse ordering now

        let mut lastnumber = 0;
        let mut lastindex = None;
        for (i, c) in line.chars().rev().enumerate() {
            if c.is_digit(10) {
                let num = c.to_digit(10).unwrap();
                lastnumber = num;
                lastindex = Some(line.chars().count() - 1 - i);
                break;
            }
        }
        for number_name in map.keys() {
            let mut matches: Vec<usize> = line.match_indices(number_name).map(|m| m.0).collect();
            matches.sort();
            if matches.len() > 0 {
                let i = matches.last().unwrap();
                if lastindex.is_none() || *i > lastindex.unwrap() {
                    lastnumber = *map.get(number_name).unwrap();
                    lastindex = Some(*i);
                }
            }
        }
        assert!(lastindex.is_some());
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
