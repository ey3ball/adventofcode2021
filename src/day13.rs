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
                    ' '
                }
            }).collect();
        println!("{}", row);
    }
    println!("");
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

pub fn paperfold(sheet: &HashSet<(i32, i32)>, axis: char, value: i32) -> HashSet<(i32, i32)> {
    sheet
        .iter()
        .filter(|(x,y)| {
            if axis == 'x' {
                x != &value
            } else {
                y != &value
            }
        })
        .map(|(x,y)| {
            if axis == 'x' && x > &value {
                (2 * value - x, *y)
            } else if axis == 'y' && y > &value {
                (*x, 2 * value - y)
            } else {
                (*x, *y)
            }
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn part1(input: &Parsed) -> i32 {
    let sheet = paperfold(&input.0, input.1[0].0, input.1[0].1);
    sheet.iter().count() as i32
}

#[aoc(day13, part2)]
pub fn part2(input: &Parsed) -> i32 {
    let sheet = input.1
        .iter()
        .fold(input.0.clone(), |sheet, instructions| {
            paperfold(&sheet, instructions.0, instructions.1)
        });
    display(&sheet);
    0
}
