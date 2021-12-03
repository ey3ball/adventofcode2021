#[aoc_generator(day3)]
pub fn generator(input: &str) -> (u32, Vec<u32>) {
    (input.lines().nth(0).unwrap().len() as u32,
     input.lines().map(|x| u32::from_str_radix(x, 2).unwrap()).collect())
}


#[aoc(day3, part1)]
pub fn part1((len, report): &(u32, Vec<u32>)) -> u32 {
    let max = report.len() as u32;
    let mut most_common_bits = 0;
    for pos in 0..*len {
        let count: u32 = report.iter()
            .map(|x| (x >> pos) & 1)
            .sum();
        if count * 2 >= max {
            most_common_bits |= 1 << pos
        }
    }
    most_common_bits * (!most_common_bits & ((1 << len) - 1))
}
