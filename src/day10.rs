use std::collections::HashMap;

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|f| f.chars().collect()).collect()
}

pub fn score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Unexpected char")
    }
}

pub fn score_auto(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Unexpected char")
    }
}


pub fn parse(line: &Vec<char>) -> Result<Vec<char>, usize> {
    let closed_by: HashMap<char, char> = HashMap::from_iter([
            ('(', ')'),
            ('[', ']'),
            ('{', '}'),
            ('<', '>')
        ].iter().copied());
    let opened_by: HashMap<char, char> = closed_by.iter().map(|(&k,&v)| (v,k)).collect();
    let mut chunks: Vec<Vec<char>> = vec![];
    let mut state: Vec<char> = vec![];
    let mut chunk: Vec<char> = vec![];

    for &c in line {
        // Opening Token
        if closed_by.contains_key(&c) {
            state.push(c);
            chunk.push(c);
        // Closing token
        } else if opened_by.contains_key(&c) {
            match state.pop() {
                Some(s) if s == opened_by[&c] => chunk.push(c),
                _ => {
                    return Err(score(c))
                },
            };
        } else {
            panic!("Unknown token");
        }

        if state.is_empty() {
            if chunk.is_empty() {
                panic!("empty chunk ?");
            }
            chunks.push(chunk);
            chunk = vec![];
        }
    };
    if state.is_empty() {
        chunks.push(chunk);
    }
    Ok(state)
}

#[aoc(day10, part1)]
pub fn part1(input: &Vec<Vec<char>>) -> usize {
    input
        .iter()
        .map(|l| parse(l))
        .filter(|p| p.is_err())
        .map(|p| p.unwrap_err())
        .sum()
}

#[aoc(day10, part2)]
pub fn part2(input: &Vec<Vec<char>>) -> usize {
    let closed_by: HashMap<char, char> = HashMap::from_iter([
            ('(', ')'),
            ('[', ']'),
            ('{', '}'),
            ('<', '>')
        ].iter().copied());

    let mut scores: Vec<usize> = input
        .iter()
        .map(|l| parse(l))
        .filter(|p| p.is_ok())
        .map(|p| {
            let state = p.unwrap();
            state
                .iter()
                .rev()
                .map(|c| closed_by[c])
                .fold(0, |score, c| {
                    score * 5 + score_auto(c)
                })
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}
