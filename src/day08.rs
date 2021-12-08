type Displays = Vec<(Vec<String>, Vec<String>)>;

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Displays {
    input
        .lines()
        .map(|l| {
            let (tests, digits) = l.split_once(" | ").unwrap();
            (tests.split(" ").map(|s| s.to_owned()).collect(),
             digits.split(" ").map(|s| s.to_owned()).collect())
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn part1(displays: &Displays) -> usize {
    let unique_lens = vec![2, 4, 3, 7];

    displays.iter()
        .flat_map(|(test_seq, digits)| digits.iter())
        .filter(|digits| unique_lens.contains(&digits.len()))
        .count()
}
