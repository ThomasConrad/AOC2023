use hashbrown::HashMap;

#[derive(Debug)]
enum Res {
    Rule(String),
    End(bool),
}

#[derive(Debug)]
struct Rule {
    comp: u8,
    greater: i32,
    value: u32,
    res: Res,
}

fn parse(input: &str) -> (HashMap<&str, Vec<Rule>>, Vec<Vec<u32>>) {
    let mut chunks = input.split("\n\n");

    (
        chunks
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let mut line = line.split("{");
                let id = line.next().unwrap();
                let rulestrings: Vec<&str> = line
                    .next()
                    .unwrap()
                    .split("}")
                    .next()
                    .unwrap()
                    .split(",")
                    .collect();
                let len = rulestrings.len();
                let mut rules: Vec<Rule> = rulestrings[0..len - 1]
                    .iter()
                    .map(|s| {
                        let greater = if s.find('>').is_some() { 1 } else { -1 };
                        let mut s = s.split(|c| c == '<' || c == '>' || c == ':');
                        let comp = match s.next().unwrap().as_bytes()[0] {
                            b'x' => 0,
                            b'm' => 1,
                            b'a' => 2,
                            b's' => 3,
                            _ => panic!("Invalid comparison"),
                        };
                        let value = s.next().unwrap().parse::<u32>().unwrap();
                        let res = match s.next().unwrap() {
                            "A" => Res::End(true),
                            "R" => Res::End(false),
                            s => Res::Rule(s.to_string()),
                        };
                        Rule {
                            comp,
                            greater,
                            value,
                            res,
                        }
                    })
                    .collect();
                rules.push(Rule {
                    comp: 0,
                    greater: 0,
                    value: 0,
                    res: match rulestrings[len - 1] {
                        "A" => Res::End(true),
                        "R" => Res::End(false),
                        s => Res::Rule(s.to_string()),
                    },
                });
                (id, rules)
            })
            .collect::<HashMap<&str, Vec<Rule>>>(),
        chunks
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                line.split(|c| c == ',' || c == '=' || c == '}')
                    .enumerate()
                    .filter(|(i, _)| i % 2 == 1)
                    .map(|(_, s)| s.parse::<u32>().unwrap())
                    .collect()
            })
            .collect::<Vec<Vec<u32>>>(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rule_map, parts) = parse(input);
    Some(
        parts
            .iter()
            .map(|part| {
                let mut loc = "in";
                'outer: loop {
                    for rule in rule_map.get(loc).unwrap() {
                        if rule.greater * part[rule.comp as usize] as i32
                            > rule.greater * rule.value as i32
                            || rule.greater == 0
                        {
                            match &rule.res {
                                Res::Rule(s) => {
                                    loc = s;
                                    continue 'outer;
                                }
                                Res::End(true) => return part.iter().sum::<u32>(),
                                Res::End(false) => return 0,
                            }
                        }
                    }
                }
            })
            .sum(),
    )
}

fn get_intervals(interval: [[i64; 2]; 4], rule: &Rule) -> ([[i64; 2]; 4], [[i64; 2]; 4]) {
    let mut passing_interval = interval;
    let edge = if rule.greater == 1 { 0 } else { 1 };
    passing_interval[rule.comp as usize][0] =
        interval[rule.comp as usize][0].max(rule.value as i64 - edge);
    passing_interval[rule.comp as usize][1] =
        interval[rule.comp as usize][1].max(rule.value as i64 - edge);

    let mut remaining_interval = interval;
    remaining_interval[rule.comp as usize][0] =
        interval[rule.comp as usize][0].min(rule.value as i64 - edge);
    remaining_interval[rule.comp as usize][1] =
        interval[rule.comp as usize][1].min(rule.value as i64 - edge);

    if rule.greater == 1 {
        (passing_interval, remaining_interval)
    } else {
        (remaining_interval, passing_interval)
    }
}

fn return_combinations(
    rule: &Rule,
    interval: [[i64; 2]; 4],
    rule_map: &HashMap<&str, Vec<Rule>>,
) -> i64 {
    match &rule.res {
        Res::Rule(s) => {
            return get_combinations(s.as_str(), interval, rule_map);
        }
        Res::End(true) => return interval.iter().map(|[a, b]| b - a).product(),
        Res::End(false) => return 0,
    }
}

fn get_combinations(
    loc: &str,
    interval: [[i64; 2]; 4],
    rule_map: &HashMap<&str, Vec<Rule>>,
) -> i64 {
    let mut total = 0;
    let mut interval = interval;
    if interval.iter().map(|[a, b]| b - a).product::<i64>() == 0 {
        return 0;
    }
    for rule in rule_map.get(loc).unwrap() {
        match rule.greater {
            0 => return total + return_combinations(rule, interval, rule_map),
            1 | -1 => {
                let (passing_interval, remaining_interval) = get_intervals(interval, rule);
                interval = remaining_interval;
                total += return_combinations(rule, passing_interval, rule_map);
            }
            _ => panic!("Invalid greater"),
        }
    }
    total
}

pub fn part_two(input: &str) -> Option<i64> {
    let (rule_map, _) = parse(input);
    Some(get_combinations("in", [[0, 4000]; 4], &rule_map))
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::submit::submit(19, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::submit::submit(19, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(19114));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), None);
    }
}
