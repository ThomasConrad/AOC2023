enum SchematicPart {
    Number((u32, u32)),
    Symbol(char),
    Empty,
}

fn parse_to_matrix(input: &str) -> (u32, Vec<Vec<SchematicPart>>) {
    let mut matrix = Vec::new();
    let mut id = 0;
    let mut last_idx = (0, 0);
    for (linenum, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        for (i, c) in chars.iter().enumerate() {
            match c {
                '.' => row.push(SchematicPart::Empty),
                '0'..='9' => {
                    //scan left and right for full number
                    let mut idx = i;
                    while idx > 0 && chars[idx - 1].is_ascii_digit() {
                        idx -= 1;
                    }
                    let mut num = String::new();
                    if last_idx != (linenum, idx) {
                        id += 1;
                        last_idx = (linenum, idx);
                    }
                    while idx < chars.len() && chars[idx].is_ascii_digit() {
                        num.push(chars[idx]);
                        idx += 1;
                    }
                    row.push(SchematicPart::Number((id, num.parse::<u32>().unwrap())));
                }
                _ => row.push(SchematicPart::Symbol(c.clone())),
            }
        }
        matrix.push(row);
    }
    (id + 1, matrix)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (max_id, matrix) = parse_to_matrix(input);
    let mut nums = vec![0; max_id as usize];
    for (x, row) in matrix.iter().enumerate() {
        for (y, part) in row.iter().enumerate() {
            if matches!(part, SchematicPart::Symbol(_)) {
                //search in a 3x3 grid around the symbol for numbers
                for i in -1..=1 {
                    for j in -1..=1 {
                        if x as i32 + i < 0
                            || y as i32 + j < 0
                            || x as i32 + i >= matrix.len() as i32
                            || y as i32 + j >= matrix[x].len() as i32
                        {
                            continue;
                        }
                        if let SchematicPart::Number((id, num)) =
                            matrix[(x as i32 + i) as usize][(y as i32 + j) as usize]
                        {
                            nums[id as usize] = num;
                        }
                    }
                }
            }
        }
    }
    Some(nums.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, matrix) = parse_to_matrix(input);
    let mut result = 0;
    for (x, row) in matrix.iter().enumerate() {
        for (y, part) in row.iter().enumerate() {
            if matches!(part, SchematicPart::Symbol('*')) {
                //search in a 3x3 grid around the symbol for numbers now finding two gear numbers
                let mut part_numbers = Vec::new();
                let mut lastid = None;
                for i in -1..=1 {
                    for j in -1..=1 {
                        if x as i32 + i < 0
                            || y as i32 + j < 0
                            || x as i32 + i >= matrix.len() as i32
                            || y as i32 + j >= matrix[x].len() as i32
                        {
                            continue;
                        }
                        if let SchematicPart::Number((id, num)) =
                            matrix[(x as i32 + i) as usize][(y as i32 + j) as usize]
                        {
                            if lastid.is_none() || lastid.unwrap() != id {
                                part_numbers.push(num);
                                lastid = Some(id);
                            }
                        }
                    }
                }
                if part_numbers.len() == 2 {
                    result += part_numbers.iter().product::<u32>();
                }
            }
        }
    }
    Some(result)
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::submit::submit(3, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::submit::submit(3, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(4361));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(467835));
    }
}
