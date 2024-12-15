use std::collections::HashMap;

use advent_of_code::helpers::get_safe;

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

fn flood(
    plants: &Vec<Vec<u8>>,
    groups: &mut Vec<Vec<Option<usize>>>,
    id: usize,
    [x, y]: [isize; 2],
) -> Option<(u32, u32)> {
    if groups[y as usize][x as usize].is_some() {
        return None;
    }

    groups[y as usize][x as usize] = Some(id);
    let plant_type = plants[y as usize][x as usize];

    let mut nbr_count = 0;

    let nbrs = [[0, 1], [1, 0], [0, -1], [-1, 0]]
        .into_iter()
        .filter_map(|[dx, dy]| match get_safe([x + dx, y + dy], plants) {
            Some(&nbr_type) if nbr_type == plant_type => {
                nbr_count += 1;
                return flood(plants, groups, id, [x + dx, y + dy]);
            }
            _ => return None,
        })
        .collect::<Vec<_>>();

    Some(
        nbrs.iter()
            .fold((1, 4 - nbr_count), |(a, p), (na, np)| (a + na, p + np)),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let plants = parse(input);
    let mut groups = vec![vec![None; plants[0].len()]; plants.len()];

    (0..plants.len())
        .flat_map(|y| (0..plants[y].len()).map(move |x| [x as isize, y as isize]))
        .filter_map(|pos| flood(&plants, &mut groups, 0, pos))
        .map(|(a, p)| a * p)
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let plants = parse(input);
    let mut groups = vec![vec![None; plants[0].len()]; plants.len()];

    let mut area_vertexcount_map = (0..plants.len())
        .flat_map(|y| (0..plants[y].len()).map(move |x| [x as isize, y as isize]))
        .enumerate()
        .filter_map(|(id, pos)| flood(&plants, &mut groups, id, pos).map(|(a, _)| (id, (a, 0))))
        .collect::<HashMap<_, _>>();

    (0..plants.len())
        .flat_map(|y| (0..plants[y].len()).map(move |x| [x as isize, y as isize]))
        .for_each(|[x, y]| {
            let id = get_safe([x, y], &groups).unwrap().unwrap();
            let vertex_count = [[0, 0], [0, 1], [1, 0], [1, 1]]
                .iter()
                .map(|block_start| {
                    let block_count = [[0, 0], [0, 1], [1, 0], [1, 1]]
                        .iter()
                        .filter(|block_pos| {
                            get_safe(
                                [
                                    x + block_start[0] + block_pos[0] - 1,
                                    y + block_start[1] + block_pos[1] - 1,
                                ],
                                &groups,
                            ) == Some(&Some(id))
                        })
                        .count() as u32;

                    match block_count {
                        1 => 3,
                        2 => {
                            //special case, must if they are diagonal
                            let c1 =
                                get_safe([x + block_start[0] - 1, y + block_start[1] - 1], &groups);
                            let c2 = get_safe([x + block_start[0], y + block_start[1]], &groups);
                            if c1 == Some(&Some(id)) && c2 == Some(&Some(id)) {
                                return 3;
                            }
                            let c3 =
                                get_safe([x + block_start[0], y + block_start[1] - 1], &groups);
                            let c4 =
                                get_safe([x + block_start[0] - 1, y + block_start[1]], &groups);

                            if c3 == Some(&Some(id)) && c4 == Some(&Some(id)) {
                                return 3;
                            }
                            0
                        }
                        3 => 1,
                        4 => 0,
                        _ => unreachable!(),
                    }
                })
                .sum::<u32>();
            area_vertexcount_map.get_mut(&id).unwrap().1 += vertex_count;
        });

    area_vertexcount_map
        .values()
        .map(|(a, p)| a * (p / 3))
        .sum::<u32>()
        .into()
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
        assert_eq!(part_one(&input), Some(140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(80));
    }
}
