#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<i32> {
    input.split(',').map(|f| f.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
pub fn part1(crab_hpos: &[i32]) -> i32 {
    let mut hpos = crab_hpos.to_owned();
    hpos.sort_unstable();

    let target = hpos[hpos.len() / 2];
    hpos.iter().map(|&h| (h - target).abs()).sum()
}

#[aoc(day7, part2)]
pub fn part2(hpos: &[i32]) -> i32 {
    let best = hpos.iter().sum::<i32>() / hpos.len() as i32;
    println!("best {}", best);
    let diff: Vec<i32> = hpos
        .iter()
        .map(|&h| (h - best).abs() * ((h - best).abs() + 1) / 2)
        .collect();
    diff.iter().sum()
}
