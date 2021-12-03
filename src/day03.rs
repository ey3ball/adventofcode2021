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

pub fn next_rating(ratings: &[u32], pos: u32, o2: u32) -> Vec<u32> {
    // Find most common bit for msb
    let max = ratings.len() as u32;
    let count: u32 = ratings.iter()
        .map(|x| (x >> (pos - 1) & 1))
        .sum();
    let most_common_bit = (if count * 2 >= max { o2 } else { 1 - o2 }) << (pos - 1);
    println!("{}", most_common_bit);
    println!("{}", count);
    println!("{}", (1 << pos - 1));

    ratings.iter().filter(|&x| ((x & (1 << pos - 1)) ^ most_common_bit) == 0).copied().collect()
}

#[aoc(day3, part2)]
pub fn part2((len, report): &(u32, Vec<u32>)) -> u32 {
    let max = report.len() as u32;
    println!("{:?}", report);

    // Find most common bit for msb
    let count: u32 = report.iter()
        .map(|x| (x >> (len - 1) & 1))
        .sum();
    let most_common_bit = (if count * 2 >= max { 1 } else { 0 }) << (len - 1);
    println!("{}", most_common_bit);
    println!("{}", (most_common_bit << (len - 1)));
    println!("{}", (1 << (len - 1)));

    // parition input according to msb
    let (mut o2_ratings, mut co2_ratings): (Vec<u32>, Vec<u32>)
        = report.iter().partition(|&x| (x & (1 << len - 1)) ^ most_common_bit == 0);

    println!("O2 ratings {:?}", o2_ratings);
    for i in (1..*len).rev() {
        o2_ratings = next_rating(&o2_ratings, i, 1);
        println!("{:?}", o2_ratings);
        if o2_ratings.len() == 1 {
            break
        }
    }

    println!("CO2 ratings {:?}", co2_ratings);
    for i in (1..*len).rev() {
        co2_ratings = next_rating(&co2_ratings, i, 0);
        println!("{:?}", co2_ratings);
        if co2_ratings.len() == 1 {
            break
        }
    }

    o2_ratings[0] * co2_ratings[0]
}
