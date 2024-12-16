/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

#[macro_export]
macro_rules! skip_fail {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                warn!("An error: {}; skipped.", e);
                continue;
            }
        }
    };
}

//Safe 2d getter
pub fn get_safe<T: Copy>(coord: [isize; 2], graph: &[Vec<T>]) -> Option<&T> {
    if coord[0] < 0
        || coord[1] < 0
        || coord[1] as usize >= graph.len()
        || coord[0] as usize >= graph[0].len()
    {
        return None;
    }
    let y: usize = coord[1] as usize;
    let x: usize = coord[0] as usize;
    Some(&graph[y][x])
}

pub fn gcd(n1: u32, n2: u32) -> u32 {
    let mut x;
    let mut y;
    if n1 > n2 {
        x = n1;
        y = n2;
    } else {
        x = n2;
        y = n1;
    }

    let mut rem = x % y;

    while rem != 0 {
        x = y;
        y = rem;
        rem = x % y;
    }
    y
}
