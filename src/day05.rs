use regex::Regex;
use std::collections::HashMap;
type Coords = (i32, i32);

#[derive(Debug)]
pub struct Lines {
    origin: Coords,
    ending: Coords,
    eq_slope: (i32, i32),
    eq_origin: i32,
}

impl Lines {
    fn new(coords: Vec<i32>) -> Lines {
        let eq_slope = (coords[3] - coords[1], coords[2] - coords[0]);
        let eq_origin = if eq_slope.1 == 0 {
            0
        } else {
            coords[1] - ((coords[0] * eq_slope.0) / eq_slope.1)
        };

        Lines {
            origin: (coords[0], coords[1]),
            ending: (coords[2], coords[3]),
            eq_origin,
            eq_slope,
        }
    }

    fn y(&self, x: i32) -> i32 {
        x * self.eq_slope.0 / self.eq_slope.1 + self.eq_origin
    }

    fn range(&self, start: i32, finish: i32) -> std::ops::RangeInclusive<i32> {
        if start > finish {
            finish..=start
        } else {
            start..=finish
        }
    }

    fn is_straight(&self) -> bool {
        return self.eq_slope.0 == 0 || self.eq_slope.1 == 0;
    }

    fn trace(&self) -> Vec<Coords> {
        if self.eq_slope.0 == 0 {
            self.range(self.origin.0, self.ending.0)
                .map(|x| (x, self.origin.1))
                .collect()
        } else if self.eq_slope.1 == 0 {
            self.range(self.origin.1, self.ending.1)
                .map(|y| (self.origin.0, y))
                .collect()
        } else {
            self.range(self.origin.0, self.ending.0)
                .map(|x| (x, self.y(x)))
                .collect()
        }
    }
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<Lines> {
    let re = Regex::new(r"^([0-9]*),([0-9]*) -> ([0-9]*),([0-9]*)").unwrap();
    input
        .lines()
        .map(|l| {
            let c = re.captures(l).unwrap();
            Lines::new(
                c.iter()
                    .skip(1)
                    .map(|c| c.unwrap().as_str().parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(lines: &Vec<Lines>) -> usize {
    let mut map: HashMap<Coords, u32> = HashMap::new();

    for l in lines.iter().filter(|l| l.is_straight()) {
        l.trace().iter().for_each(|coord| {
            map.insert(*coord, map.get(coord).unwrap_or(&0) + 1);
        });
    }

    map.values().filter(|&&v| v >= 2).count()
}

#[aoc(day5, part2)]
pub fn part2(lines: &Vec<Lines>) -> usize {
    let mut map: HashMap<Coords, u32> = HashMap::new();

    for l in lines.iter() {
        l.trace().iter().for_each(|coord| {
            map.insert(*coord, map.get(coord).unwrap_or(&0) + 1);
        });
    }

    map.values().filter(|&&v| v >= 2).count()
}
