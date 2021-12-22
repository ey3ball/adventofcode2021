use std::collections::HashSet;
use std::ops::Range;
type R = (i64,i64);

#[aoc_generator(day22)]
pub fn parse(input: &str) -> Vec<(bool, (R,R,R))> {
    input.lines().map(
        |l| {
            let (on,coords) = l.split_once(" ").unwrap();
            let on = on == "on";
            let mut coords = coords
                .split(",")
                .map(|c| {
                    let (_, range) = c.split_once('=').unwrap();
                    let (from, to) = range.split_once("..").unwrap();
                    (from.parse().unwrap(), to.parse().unwrap())
                });
            (on, (coords.next().unwrap(), coords.next().unwrap(), coords.next().unwrap()))
        }
    ).collect()
}

pub fn small_range(range: &R) -> Range<i64> {
    if range.0 < -50 && range.1 < -50 {
        return 0..0
    } else if range.0 > 50 && range.1 > 50 {
        return 0..0
    } else {
        std::cmp::max(range.0, -50)..(std::cmp::min(range.1, 50) + 1)
    }
}

#[aoc(day22, part1)]
pub fn part1(steps: &[(bool, (R,R,R))]) -> usize {
    let mut state: HashSet<(i64, i64, i64)> = HashSet::new();

    for (on, (r_x, r_y, r_z)) in steps {
        iproduct!(small_range(r_x), small_range(r_y), small_range(r_z))
            .for_each(|coord| {
                if *on {
                    state.insert(coord);
                } else {
                    state.remove(&coord);
                }
            });
    }
    state.len()
}
