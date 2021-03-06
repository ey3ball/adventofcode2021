use std::collections::HashSet;
type Disp = (Vec<HashSet<char>>, Vec<HashSet<char>>);

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Vec<Disp> {
    input
        .lines()
        .map(|l| {
            let (tests, digits) = l.split_once(" | ").unwrap();
            (
                tests.split(' ').map(|s| s.chars().collect()).collect(),
                digits.split(' ').map(|s| s.chars().collect()).collect(),
            )
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn part1(displays: &[Disp]) -> usize {
    let unique_lens = vec![2, 4, 3, 7];

    displays
        .iter()
        .flat_map(|(_test_seq, digits)| digits.iter())
        .filter(|digits| unique_lens.contains(&digits.len()))
        .count()
}

pub fn map_digits(test_seq: &[HashSet<char>]) -> Vec<HashSet<char>> {
    let mut sequences: [Option<HashSet<char>>; 10] = Default::default();

    while sequences.iter().any(|item| item.is_none()) {
        for seq in test_seq.iter() {
            match seq.len() {
                2 => sequences[1] = Some(seq.clone()),
                3 => sequences[7] = Some(seq.clone()),
                4 => sequences[4] = Some(seq.clone()),
                5 => {
                    // either 2 / 3 / 5

                    // 3 has two segments in common with 1
                    if let Some(one) = &sequences[1] {
                        if one.intersection(seq).count() == 2 {
                            sequences[3] = Some(seq.clone())
                        }
                    }

                    // 2 two segments in common 4
                    if let Some(four) = &sequences[4] {
                        if four.intersection(seq).count() == 2 {
                            sequences[2] = Some(seq.clone())
                        }
                    }

                    // once we know 2 / 3 the other one is 5
                    if let Some((two, three)) = sequences[2].clone().zip(sequences[3].clone()) {
                        if two != *seq && three != *seq {
                            sequences[5] = Some(seq.clone())
                        }
                    }
                }
                6 => {
                    // either 0, 6, 9 ()
                    // 4 is embedded into 9
                    if let Some(four) = &sequences[4] {
                        if four.intersection(seq).count() == 4 {
                            sequences[9] = Some(seq.clone())
                        }
                    }

                    // 8-9+5 == 6
                    if let Some(((eight, nine), five)) = sequences[8]
                        .clone()
                        .zip(sequences[9].clone())
                        .zip(sequences[5].clone())
                    {
                        if &(&eight ^ &nine) | &five == *seq {
                            sequences[6] = Some(seq.clone())
                        }
                    }

                    // once we know 6 / 9 the other one is 0
                    if let Some((six, nine)) = sequences[6].clone().zip(sequences[9].clone()) {
                        if six != *seq && nine != *seq {
                            sequences[0] = Some(seq.clone())
                        }
                    }
                }
                7 => sequences[8] = Some(seq.clone()),
                _ => panic!(),
            }
        }
    }

    return sequences
        .iter()
        .map(|s| s.as_ref().unwrap())
        .cloned()
        .collect();
}

#[aoc(day8, part2)]
pub fn part2(displays: &[Disp]) -> usize {
    displays
        .iter()
        .map(|(test_seq, digits)| {
            let map = map_digits(test_seq);
            let decoded = digits
                .iter()
                .map(|d| map.iter().position(|e| e == d).unwrap())
                .collect::<Vec<usize>>();
            decoded[0] * 1000 + decoded[1] * 100 + decoded[2] * 10 + decoded[3]
        })
        .sum()
}
