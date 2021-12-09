use std::collections::HashSet;

#[derive(Debug)]
pub struct HeightMap {
    xmax: i32,
    ymax: i32,
    arr: Vec<i8>,
}

impl HeightMap {
    fn pt(&self, index: usize) -> (i32, i32) {
        let index = index as i32;
        (index % self.xmax, (index - (index % self.xmax)) / self.xmax)
    }
    fn idx(&self, pt: (i32, i32)) -> usize {
        (pt.0 + pt.1 * self.xmax).try_into().unwrap()
    }

    fn val(&self, pt: (i32, i32)) -> i8 {
        self.arr[self.idx(pt)]
    }

    fn neigh(&self, pt: (i32, i32)) -> impl Iterator<Item = (i32, i32)> + '_ {
        (pt.0 - 1..=pt.0 + 1)
            .filter(|&x| (0..self.xmax).contains(&x))
            .flat_map(move |x| {
                (pt.1 - 1..=pt.1 + 1)
                    .filter(|&y| (0..self.ymax).contains(&y))
                    .map(move |y| (x, y))
            })
            .filter(move |&(x_, y_)| (x_ == pt.0) ^ (y_ == pt.1))
    }

    fn neigh_vals(&self, pt: (i32, i32)) -> impl Iterator<Item = i8> + '_ {
        return self.neigh(pt).map(|neigh_pt| self.arr[self.idx(neigh_pt)]);
    }
}

#[aoc_generator(day9)]
pub fn generator(input: &str) -> HeightMap {
    let (cols, lines) = (input.lines().next().unwrap().len(), input.lines().count());
    let map: Vec<i8> = input
        .lines()
        .flat_map(|l| l.chars())
        .map(|c| c.to_digit(10).unwrap() as i8)
        .collect();
    HeightMap {
        xmax: cols as i32,
        ymax: lines as i32,
        arr: map,
    }
}

#[aoc(day9, part1)]
pub fn part1(input: &HeightMap) -> isize {
    input
        .arr
        .iter()
        .enumerate()
        .filter(|(i, &val)| {
            input
                .neigh_vals(input.pt(*i))
                .all(|neigh_val| neigh_val > val)
        })
        .map(|(_i, val)| (val + 1) as isize)
        .sum()
}

pub fn grow(input: &HeightMap, basin: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let grown: HashSet<(i32, i32)> = basin
        .iter()
        .flat_map(|&pt| input.neigh(pt).filter(|&neigh| input.val(neigh) != 9))
        .collect();
    if basin == &basin | &grown {
        basin
    } else {
        grow(input, &basin | &grown)
    }
}

#[aoc(day9, part2)]
pub fn part2(input: &HeightMap) -> usize {
    let low_points: Vec<(i32, i32)> = input
        .arr
        .iter()
        .enumerate()
        .filter(|(i, &val)| {
            input
                .neigh_vals(input.pt(*i))
                .all(|neigh_val| neigh_val > val)
        })
        .map(|(i, _val)| input.pt(i))
        .collect();

    let mut basin_sizes: Vec<usize> = low_points
        .iter()
        .map(|&pt| {
            let mut basin = HashSet::new();
            basin.insert(pt);
            basin = grow(input, basin);
            basin.iter().count()
        })
        .collect();

    basin_sizes.sort();

    basin_sizes.iter().rev().take(3).product()
}
