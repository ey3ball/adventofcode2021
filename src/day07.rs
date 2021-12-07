#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<u32> {
    input
        .split(",")
        .map(|f| f.parse().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(crab_hpos: &Vec<u32>) -> i32 {
    let mut hpos = crab_hpos.clone();
    hpos.sort();

    let target = hpos[hpos.len() / 2];
    hpos.iter().map(|&h| i32::abs(h as i32 - target as i32)).sum()
}

#[aoc(day7, part2)]
pub fn part2(hpos: &Vec<u32>) -> i32 {
    let best = hpos.iter().sum::<u32>() as usize / hpos.len();
    let diff: Vec<i32> = hpos.iter().map(|&h| {
        let dst = i32::abs(h as i32 - best as i32);
        dst * (dst + 1) / 2
    }).collect();
    diff.iter().sum::<i32>()
}
