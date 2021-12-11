use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct OctoMap {
    xmax: i32,
    ymax: i32,
    arr: Vec<i32>,
}

impl OctoMap {
    fn pt(&self, index: usize) -> (i32, i32) {
        let index = index as i32;
        (index % self.xmax, (index - (index % self.xmax)) / self.xmax)
    }
    fn idx(&self, pt: (i32, i32)) -> usize {
        (pt.0 + pt.1 * self.xmax).try_into().unwrap()
    }

    fn val(&self, pt: (i32, i32)) -> &i32 {
        &self.arr[self.idx(pt)]
    }

    fn inc(&mut self, pt: (i32, i32)) -> () {
        let idx = self.idx(pt);
        self.arr[idx] += 1
    }

    fn neigh(&self, pt: (i32, i32)) -> Vec<(i32, i32)> {
        (pt.0 - 1..=pt.0 + 1)
            .filter(|&x| (0..self.xmax).contains(&x))
            .flat_map(move |x| {
                (pt.1 - 1..=pt.1 + 1)
                    .filter(|&y| (0..self.ymax).contains(&y))
                    .map(move |y| (x, y))
            })
            .filter(|&(x_, y_)| !(x_ == pt.0 && y_ == pt.1))
            .collect()
    }

    fn debug(&self) {
        println!("map");
        for y in 0..self.ymax {
            println!(
                "{}",
                ((0..self.xmax)
                    .map(|x| self.val((x, y)))
                    .map(|v| char::from_digit(*v as u32, 10).unwrap())
                    .collect::<String>())
            )
        }
    }

    //fn neigh_vals(&self, pt: (i32, i32)) -> impl Iterator<Item = i32> + '_ {
    //    return self.neigh(pt).map(|neigh_pt| self.arr[self.idx(neigh_pt)]);
    //}
}

#[aoc_generator(day11)]
pub fn generator(input: &str) -> OctoMap {
    let (cols, lines) = (input.lines().next().unwrap().len(), input.lines().count());
    let map: Vec<i32> = input
        .lines()
        .flat_map(|l| l.chars())
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    OctoMap {
        xmax: cols as i32,
        ymax: lines as i32,
        arr: map,
    }
}

pub fn step(n_1: &OctoMap) -> (usize, OctoMap) {
    let mut n = n_1.clone();

    n.arr.iter_mut().for_each(|en| *en += 1);

    let mut flashed: HashSet<usize> = HashSet::new();

    loop {
        let flash: HashSet<usize> = n
            .arr
            .iter()
            .enumerate()
            .filter(|(_, en)| **en > 9)
            .map(|(idx, _)| idx)
            .collect();

        //println!("flash: {:?}", n.arr);
        //println!("{:?}", &flash - &flashed);
        if &flash | &flashed == flashed {
            break;
        }

        (&flash - &flashed)
            .iter()
            .for_each(|idx| n.neigh(n.pt(*idx)).iter().for_each(|pt| n.inc(*pt)));

        flashed = &flashed | &flash;
    }

    println!("{:?}", flashed);

    n.arr
        .iter_mut()
        .filter(|energy| **energy > 9)
        .for_each(|energy| *energy = 0);

    (flashed.iter().count(), n)
}

#[aoc(day11, part1)]
pub fn part1(input: &OctoMap) -> usize {
    let mut flash = 0;
    let mut map = input.clone();
    for i in 0..100 {
        map.debug();
        let (inc, new_map) = step(&map);
        map = new_map;
        flash += inc
    }
    flash
}

#[aoc(day11, part2)]
pub fn part2(input: &OctoMap) -> usize {
    let mut flash = 0;
    let mut map = input.clone();
    let mut final_step = 0;
    for steps in 0..10000 {
        map.debug();
        let (inc, new_map) = step(&map);
        map = new_map;
        if inc == 100 {
            final_step = steps;
            break;
        }
        flash += inc
    }
    final_step
}
