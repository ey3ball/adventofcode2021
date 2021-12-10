use wasm_bindgen::prelude::*;
type Moves = (String, i64);

#[aoc_generator(day2)]
#[wasm_bindgen]
pub fn generator(input: &str) -> Vec<Moves> {
    input
        .lines()
        .map(|x| {
            let (direction, amount) = x.split_once(" ").unwrap();
            let amount: i64 = amount.parse().unwrap();
            (direction.to_owned(), amount)
        })
        .collect()
}

#[aoc(day2, part1)]
#[wasm_bindgen]
pub fn part1(moves: &[Moves]) -> i64 {
    let pos = moves
        .iter()
        .fold((0, 0), |(x, y), (dir, val)| match dir.as_str() {
            "forward" => (x + val, y),
            "backward" => (x - val, y),
            "down" => (x, y + val),
            "up" => (x, y - val),
            _ => panic!("unsupported move"),
        });
    pos.0 * pos.1
}

#[aoc(day2, part2)]
pub fn part2(moves: &[Moves]) -> i64 {
    let pos = moves
        .iter()
        .fold((0, 0, 0), |(x, y, aim), (dir, val)| match dir.as_str() {
            "forward" => (x + val, y + val * aim, aim),
            "backward" => (x - val, y, aim),
            "down" => (x, y, aim + val),
            "up" => (x, y, aim - val),
            _ => panic!("unsupported move"),
        });
    pos.0 * pos.1
}
