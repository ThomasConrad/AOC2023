pub fn part_one(input: &str) -> Option<u32> {
    let map = ["red", "green", "blue"];
    let avail = [12, 13, 14];
    let mut id_sum = 0;
    'outer: for (id, line) in input.lines().enumerate() {
        let mut result = [0; 3];
        let words: Vec<&str> = line.split_whitespace().collect();
        for (i, col_string) in map.iter().enumerate() {
            for (j, word) in words.iter().enumerate() {
                if word.contains(col_string) {
                    let num = words[j - 1].parse::<u32>().unwrap();
                    result[i] = std::cmp::max(result[i], num);
                }
            }
        }
        for (num_avail, num_req) in avail.iter().zip(result.iter()) {
            if num_req > num_avail {
                continue 'outer;
            }
        }
        id_sum += (id + 1) as u32;
    }
    Some(id_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = ["red", "green", "blue"];
    let mut power_sum = 0;
    for line in input.lines() {
        let mut result = [0; 3];
        let words: Vec<&str> = line.split_whitespace().collect();
        for (i, col_string) in map.iter().enumerate() {
            for (j, word) in words.iter().enumerate() {
                if word.contains(col_string) {
                    let num = words[j - 1].parse::<u32>().unwrap();
                    result[i] = std::cmp::max(result[i], num);
                }
            }
        }
        power_sum += result.iter().product::<u32>();
    }
    Some(power_sum)
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::submit::submit(2, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::submit::submit(2, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(8));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
