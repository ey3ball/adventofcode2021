use std::collections::HashSet;
type Parsed = (HashSet<(i32, i32)>, Vec<(char, i32)>);

pub fn display(grid: &HashSet<(i32, i32)>) {
    let xmax = grid.iter().map(|(x,_y)| x).max().unwrap();
    let ymax = grid.iter().map(|(_x,y)| y).max().unwrap();

    for y in 0..=*ymax {
        let row: String = (0..=*xmax)
            .map(|x| {
                if grid.contains(&(x,y)) {
                    '#'
                } else {
                    '.'
                }
            }).collect();
        println!("{}", row);
    }
}

#[aoc_generator(day13)]
pub fn parse(input: &str) -> Parsed {
    let (dots, instructions) = input.split_once("\n\n").unwrap();
    let dots =
        dots
            .lines()
            .map(|l| {
                let (x,y) = l.split_once(",").unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();
    let instructions =
        instructions
            .lines()
            .map(|i| {
                 let (axis, value) = i[11..].split_once("=").unwrap();
                 (axis.chars().nth(0).unwrap(), value.parse().unwrap())
            })
            .collect();
    (dots, instructions)
}

#[aoc(day13, part1)]
pub fn part1(input: &Parsed) -> usize {
    println!("{:?}", input);
    display(&input.0);
    0
}

#[aoc(day13, part2)]
pub fn part2(input: &Parsed) -> usize {
    0
}
