use hashbrown::HashMap;

fn parse(input: &str) -> impl Iterator<Item = (&str, Vec<u32>)> + '_ {
    input.lines().map(|line| {
        let mut parts = line.split_whitespace();
        let code = parts.next().unwrap();
        let sizes = parts
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        (code, sizes)
    })
}

struct Arrangement {
    code: Vec<char>,
    sizes: Vec<u32>,
    cache: HashMap<(usize, usize), u64>,
}

impl Arrangement {
    fn new(code: &str, sizes: Vec<u32>) -> Self {
        Self {
            code: code.chars().collect(),
            sizes,
            cache: HashMap::new(),
        }
    }
    fn check_valid_combinations(&mut self, codeindex: usize, sizeindex: usize) -> u64 {
        if let Some(&v) = self.cache.get(&(codeindex, sizeindex)) {
            return v;
        }
        let v = self.check(codeindex, sizeindex);
        self.cache.insert((codeindex, sizeindex), v);
        v
    }

    fn check(&mut self, codeindex: usize, sizeindex: usize) -> u64 {
        //check that it's even possible and stuff
        if self.sizes.len() <= sizeindex {
            //End of the line, baby
            if self.code.len() > codeindex && self.code[codeindex..].contains(&'#') {
                return 0;
            }
            return 1;
        }
        if codeindex >= self.code.len() {
            return 0;
        }
        let code = &self.code[codeindex..];
        let sizes = &self.sizes[sizeindex..];

        match code[0] {
            '?' => {
                //scan segment size
                let max_len = code.iter().position(|&c| c == '.').unwrap_or(code.len());
                //we can try to fit in chunk
                if sizes[0] <= max_len as u32
                    && code
                        .get(sizes[0] as usize)
                        .map(|&c| c != '#')
                        .unwrap_or(true)
                {
                    return self.check_valid_combinations(
                        codeindex + sizes[0] as usize + 1,
                        sizeindex + 1,
                    ) + self.check_valid_combinations(codeindex + 1, sizeindex);
                }
                //skipping
                self.check_valid_combinations(codeindex + 1, sizeindex)
            }
            '#' => {
                //we know we have to place something here right now
                //scan for full blob
                let max_len = code.iter().position(|&c| c == '.').unwrap_or(code.len());
                if sizes[0] <= max_len as u32
                    && code
                        .get(sizes[0] as usize)
                        .map(|&c| c != '#')
                        .unwrap_or(true)
                {
                    return self.check_valid_combinations(
                        codeindex + sizes[0] as usize + 1,
                        sizeindex + 1,
                    );
                }
                0
            }
            '.' => self.check_valid_combinations(codeindex + 1, sizeindex), //jump one. Will then check if it's valid inside function
            _ => panic!("Invalid character in code: {:?}", code),
        }
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .map(|(code, sizes)| {
                let mut arr = Arrangement::new(code, sizes);
                arr.check_valid_combinations(0, 0)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .map(|(code, sizes)| {
                let code = [code; 5].join(&"?");
                let sizes = sizes.repeat(5);
                let mut arr = Arrangement::new(code.as_str(), sizes);
                arr.check_valid_combinations(0, 0)
            })
            .sum(),
    )
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::submit::submit(12, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::submit::submit(12, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(525152));
    }
}
