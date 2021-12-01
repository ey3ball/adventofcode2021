use itertools::Itertools;

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<i32> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(depths: &[i32]) -> usize {
    let d1 = &depths[0..depths.len() - 1];
    let d2 = &depths[1..depths.len()];

    d1.iter()
        .zip(d2.iter())
        .map(|(x, y)| y - x)
        .filter(|&x| x > 0)
        .count()
}

#[aoc(day1, part2)]
pub fn part2(depths: &[i32]) -> usize {
    let windowed: Vec<(i32, i32, i32)> = depths.iter().copied().tuple_windows().collect();

    let wd1 = &windowed[0..windowed.len() - 1];
    let wd2 = &windowed[1..windowed.len()];

    wd1.iter()
        .zip(wd2.iter())
        .map(|(x, y)| y.2 + y.1 + y.0 - x.2 - x.1 - x.0)
        .filter(|&x| x > 0)
        .count()
}
