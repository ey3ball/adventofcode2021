use core::ops::RangeInclusive;

//const TARGET_X: RangeInclusive<isize> = 20..=30;
//const TARGET_Y: RangeInclusive<isize> = -10..=-5;

//x=235..259, y=-118..-62

const TARGET_X: RangeInclusive<isize> = 235..=259;
const TARGET_Y: RangeInclusive<isize> = -118..=-62;


pub fn reaches_area(dx: isize, dy: isize) -> Option<isize> {
    let mut pos = (0, 0);
    let mut d = (dx, dy);
    let mut ymax = 0;
    while pos.0 <= TARGET_X.last().unwrap() && pos.1 >= TARGET_Y.clone().nth(0).unwrap() {
        if pos.1 > ymax {
            ymax = pos.1;
        }

        if TARGET_X.contains(&pos.0) && TARGET_Y.contains(&pos.1) {
            return Some(ymax);
        }

        pos = (pos.0 + d.0, pos.1 + d.1);
        if d.0 > 0 {
            d.0 -= 1;
        } else if d.0 < 0 {
            d.0 += 1;
        }
        d.1 -= 1;
    };
    None
}

#[aoc(day17, part1)]
pub fn part1(_input: &str) -> isize {
    let try_x = 1..1000;
    let try_y = -1000..1000;

    try_x
        .flat_map(|dx| try_y.clone().map(move |dy| (dx,dy)))
        .filter_map(|xy| reaches_area(xy.0, xy.1))
        .max()
        .unwrap()
}

#[aoc(day17, part2)]
pub fn part2(_input: &str) -> usize {
    let try_x = 1..1000;
    let try_y = -1000..1000;

    try_x
        .flat_map(|dx| try_y.clone().map(move |dy| (dx,dy)))
        .filter_map(|xy| reaches_area(xy.0, xy.1))
        .count()
}


