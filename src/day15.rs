use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Cavern {
    xmax: i32,
    ymax: i32,
    arr: Vec<i32>,
}

impl Cavern {
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
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .map(|(dx, dy)| (pt.0 + dx, pt.1 + dy))
            .filter(|(x, y)| (0..self.xmax).contains(x) && (0..self.ymax).contains(y))
            .collect()
    }

    fn neigh_idx(&self, idx: usize) -> Vec<(i32, i32)> {
        let pt = self.pt(idx);
        self.neigh(pt)
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

    fn neigh_vals(&self, idx: usize) -> Vec<(usize, i32)> {
        self.neigh_idx(idx)
            .iter()
            .map(|&pt| (self.idx(pt), self.arr[self.idx(pt)]))
            .collect()
    }
}

#[aoc_generator(day15)]
pub fn generator(input: &str) -> Cavern {
    let (cols, lines) = (input.lines().next().unwrap().len(), input.lines().count());
    let map: Vec<i32> = input
        .lines()
        .flat_map(|l| l.chars())
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    Cavern {
        xmax: cols as i32,
        ymax: lines as i32,
        arr: map,
    }
}

pub fn explore(input: &Cavern) -> i32 {
    let mut dst: HashMap<usize, Option<i32>> = HashMap::new();
    let mut to_visit: BinaryHeap<(i32, usize)> = BinaryHeap::new();
    to_visit.push((0, 0));
    dst.insert(0, Some(0));

    loop {
        let (from_risk, next_point) = match to_visit.pop() {
            Some(risk_point) => (-risk_point.0, risk_point.1),
            _ => break,
        };

        if next_point == input.arr.len() - 1 {
            break;
        }

        if from_risk != dst.get(&next_point).unwrap().unwrap() {
            continue;
        }

        input.neigh_vals(next_point).iter().for_each(|(idx, risk)| {
            match dst.entry(*idx).or_insert(None) {
                Some(val) if val <= &mut (from_risk + risk) => (),
                _ => {
                    dst.insert(*idx, Some(from_risk + risk));
                    to_visit.push((-(from_risk + risk), *idx));
                }
            };
        });
    }
    dst.get(&(input.arr.len() - 1)).unwrap().unwrap()
}

pub fn grow(input: &Cavern) -> Cavern {
    let factor = 5;
    let xmax = (input.xmax * factor) as i32;
    let ymax = (input.ymax * factor) as i32;

    let mut big = Cavern {
        xmax: xmax as i32,
        ymax: ymax as i32,
        arr: vec![0; (xmax * ymax) as usize],
    };

    for y in 0..input.ymax {
        for repeat in 0..factor {
            for x in 0..input.xmax {
                let new_idx = big.idx((x + repeat * input.xmax, y));
                let old_idx = input.idx((x, y));
                big.arr[new_idx] = (input.arr[old_idx] + repeat - 1) % 9 + 1;
            }
        }
    }

    for repeat in 1..factor {
        for y in 0..input.ymax {
            for x in 0..big.xmax {
                let from_idx = big.idx((x, y));
                let to_idx = big.idx((x, y + repeat * input.ymax));
                big.arr[to_idx] = (big.arr[from_idx] + repeat - 1) % 9 + 1;
            }
        }
    }

    big
}

#[aoc(day15, part1)]
pub fn part1(input: &Cavern) -> i32 {
    explore(input)
}

#[aoc(day15, part2)]
pub fn part2(input: &Cavern) -> i32 {
    let bigger_input = grow(input);
    explore(&bigger_input)
}
