#[derive(Clone, Debug, PartialEq)]
enum StorageBlock {
    Filled(u32, u32), // (size, id)
    Empty(u32),
}

impl StorageBlock {
    fn size(&self) -> u32 {
        match self {
            StorageBlock::Filled(size, _) => *size,
            StorageBlock::Empty(size) => *size,
        }
    }
}

fn parse_input(input: &str) -> Vec<StorageBlock> {
    input
        .chars()
        .fold((true, 0, Vec::new()), |(state, mut id, mut blocks), c| {
            let block = match (state, c.to_digit(10)) {
                (false, Some(num)) => StorageBlock::Empty(num),
                (true, Some(num)) => {
                    let block = StorageBlock::Filled(num, id);
                    id += 1;
                    block
                }
                _ => unreachable!(),
            };
            if block != StorageBlock::Empty(0) {
                blocks.push(block.clone());
            }
            (!state, id, blocks)
        })
        .2
}

pub fn part_one(input: &str) -> Option<u64> {
    let storage_blocks = parse_input(input);
    let mut optimized_storage = Vec::<StorageBlock>::new();

    let mut front_index = 0;
    let mut back_index = storage_blocks.len() - 1;

    let mut remaining = None::<StorageBlock>;
    while front_index < back_index {
        match (
            &storage_blocks[front_index],
            &storage_blocks[back_index],
            &remaining,
        ) {
            (front @ StorageBlock::Filled(_, _), _, _) => {
                optimized_storage.push(front.clone());
                front_index += 1;
            }
            (StorageBlock::Empty(space), _, Some(StorageBlock::Filled(num, id)))
            | (_, StorageBlock::Filled(num, id), Some(StorageBlock::Empty(space)))
            | (StorageBlock::Empty(space), StorageBlock::Filled(num, id), None) => {
                match space.cmp(num) {
                    std::cmp::Ordering::Equal => {
                        optimized_storage.push(StorageBlock::Filled(*num, *id));
                        remaining = None;
                        front_index += 1;
                        back_index -= 1;
                    }
                    std::cmp::Ordering::Greater => {
                        optimized_storage.push(StorageBlock::Filled(*num, *id));
                        remaining = Some(StorageBlock::Empty(space - num));
                        back_index -= 1;
                    }
                    std::cmp::Ordering::Less => {
                        optimized_storage.push(StorageBlock::Filled(*space, *id));
                        remaining = Some(StorageBlock::Filled(num - space, *id));
                        front_index += 1;
                    }
                }
            }
            (_, StorageBlock::Empty(_), _) => {
                back_index -= 1;
            }
        }
    }
    //if any remaining block is left, push it to the optimized storage
    if matches!(remaining, Some(StorageBlock::Filled(_, _))) {
        optimized_storage.push(remaining.unwrap());
    }
    Some(
        optimized_storage
            .iter()
            .fold((0, 0), |(idx, value), block| {
                if let StorageBlock::Filled(size, id) = block {
                    (
                        idx + size,
                        value
                            + *id as u64 * *size as u64 * (2 * (idx as u64) + *size as u64 - 1) / 2,
                    )
                } else {
                    unreachable!()
                }
            })
            .1,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut storage_blocks = parse_input(input);
    let mut back_index = storage_blocks.len() - 1;

    while back_index > 0 {
        //search for a block that fits this
        if let StorageBlock::Filled(size, id) = storage_blocks[back_index] {
            let block =
                storage_blocks[..back_index]
                    .iter_mut()
                    .enumerate()
                    .find(|(_, b)| match b {
                        StorageBlock::Empty(space) => *space >= size,
                        _ => false,
                    });

            //split block
            if let Some((idx, found @ StorageBlock::Empty(_))) = block {
                let found_size = found.size();
                if found_size == size {
                    *found = StorageBlock::Filled(size, id);
                } else {
                    *found = StorageBlock::Filled(size, id);
                    storage_blocks.insert(idx + 1, StorageBlock::Empty(found_size - size));
                    back_index += 1;
                }
                storage_blocks[back_index] = StorageBlock::Empty(size);
            }
        }
        back_index -= 1;
    }
    Some(
        storage_blocks
            .iter()
            .fold((0, 0), |(idx, value), block| {
                if let StorageBlock::Filled(size, id) = block {
                    (
                        idx + size,
                        value
                            + *id as u64 * *size as u64 * (2 * (idx as u64) + *size as u64 - 1) / 2,
                    )
                } else {
                    (idx + block.size(), value)
                }
            })
            .1,
    )
}

#[cfg(feature = "solve")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(feature = "submit1")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::submit::submit(9, 1, part_one, input);
}

#[cfg(feature = "submit2")]
fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::submit::submit(9, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(1928));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
