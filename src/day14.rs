use itertools::Itertools;
use std::collections::HashMap;
type Input = (Vec<char>, HashMap<(char, char), char>);

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Input {
    let (init, rules) = input.split_once("\n\n").unwrap();

    let init = init.chars().collect();
    let rules = rules
        .lines()
        .map(|l| {
            let (from, to) = l.split_once(" -> ").unwrap();
            let from = (from.chars().nth(0).unwrap(), from.chars().nth(1).unwrap());
            let to = to.chars().nth(0).unwrap();
            (from, to)
        })
        .collect();

    (init, rules)
}

pub fn stats(polymer: &Vec<char>) -> usize {
    let mut stats: Vec<(char, usize)> = polymer.iter().counts_by(|x| *x).into_iter().collect();
    stats.sort_by(|(_, count), (_, count_2)| count.partial_cmp(count_2).unwrap());
    // println!("{:?}", stats);
    stats.iter().last().unwrap().1 - stats.iter().next().unwrap().1
}

pub fn stats2(first: char, seqs: &HashMap<(char, char), usize>) -> usize {
    let mut stats: HashMap<char, usize> = HashMap::new();
    stats.insert(first, 1);
    seqs.iter().for_each(|((_, b), count)| {
        *(stats.entry(*b).or_insert(0)) += count;
    });
    let mut stats: Vec<(char, usize)> = stats.iter().map(|(k, v)| (*k, *v)).collect();
    stats.sort_by(|(_, count), (_, count_2)| count.partial_cmp(count_2).unwrap());
    // println!("{:?}", stats);
    stats.iter().last().unwrap().1 - stats.iter().next().unwrap().1
}

#[aoc(day14, part1)]
pub fn part1(input: &Input) -> usize {
    let rules = &input.1;
    let mut state = input.0.clone();
    for _i in 0..10 {
        let mut next_state: Vec<char> = vec![];
        let mut first = true;
        state.windows(2).for_each(|win| {
            let ins = rules.get(&(win[0], win[1])).unwrap();
            if first {
                next_state.push(win[0]);
            }
            first = false;
            next_state.push(*ins);
            next_state.push(win[1]);
        });
        state = next_state;
        // stats(&state);
    }
    stats(&state)
}

#[aoc(day14, part2)]
pub fn part2(input: &Input) -> usize {
    let rules = &input.1;
    let mut state: HashMap<(char, char), usize> = input
        .0
        .windows(2)
        .map(|win| (win[0], win[1]))
        .counts_by(|couple| couple)
        .into_iter()
        .collect();

    for _i in 0..40 {
        let mut next_state: HashMap<(char, char), usize> = HashMap::new();

        state.iter().for_each(|(couple, count)| {
            let ins = rules.get(&(couple.0, couple.1)).unwrap();
            let counter = next_state.entry((couple.0, *ins)).or_insert(0);
            *counter += count;
            let counter = next_state.entry((*ins, couple.1)).or_insert(0);
            *counter += count;
        });
        state = next_state;
        // stats2(input.0[0], &state);
    }
    stats2(input.0[0], &state)
}
