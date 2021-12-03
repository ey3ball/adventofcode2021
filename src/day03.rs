#[aoc_generator(day3)]
pub fn generator(input: &str) -> (u32, Vec<u32>) {
    (
        input.lines().nth(0).unwrap().len() as u32,
        input
            .lines()
            .map(|x| u32::from_str_radix(x, 2).unwrap())
            .collect(),
    )
}

#[aoc(day3, part1)]
pub fn part1((len, report): &(u32, Vec<u32>)) -> u32 {
    let max = report.len() as u32;
    let mut most_common_bits = 0;
    for pos in 0..*len {
        let count: u32 = report.iter().map(|x| (x >> pos) & 1).sum();
        if count * 2 >= max {
            most_common_bits |= 1 << pos
        }
    }
    most_common_bits * (!most_common_bits & ((1 << len) - 1))
}

pub fn next_ratings(ratings: &[u32], pos: u32, o2: u32) -> Vec<u32> {
    let threshold = ratings.len() as u32;
    let msb_count: u32 = ratings.iter().map(|x| (x >> pos & 1)).sum();
    let most_common_bit = (if msb_count * 2 >= threshold {
        o2
    } else {
        1 - o2
    }) << pos;

    ratings
        .iter()
        .filter(|&x| ((x & (1 << pos)) ^ most_common_bit) == 0)
        .copied()
        .collect()
}

pub fn find_rating(width: u32, ratings: &[u32], o2: u32) -> u32 {
    let mut ratings: Vec<u32> = ratings.to_vec();
    for i in (0..width).rev() {
        ratings = next_ratings(&ratings, i, o2);
        if ratings.len() == 1 {
            break;
        }
    }
    ratings[0]
}

#[aoc(day3, part2)]
pub fn part2((len, report): &(u32, Vec<u32>)) -> u32 {
    let o2_rating = find_rating(*len, report, 1);
    let co2_rating = find_rating(*len, report, 0);

    o2_rating * co2_rating
}
