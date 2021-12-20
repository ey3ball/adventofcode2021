use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Map {
    xmin: i32,
    ymin: i32,
    xmax: i32,
    ymax: i32,
    codec: HashMap<usize, bool>,
    arr: HashSet<(i32, i32)>,
    bg: bool
}

impl Map {
    fn grow(&mut self) {
        self.xmin -= 1;
        self.xmax += 1;
        self.ymin -= 1;
        self.ymax += 1;
    }

    fn neigh(&self, pt: (i32, i32)) -> Vec<(i32, i32)> {
        (pt.1 - 1..=pt.1 + 1)
            .flat_map(move |y| {
                (pt.0 - 1..=pt.0 + 1)
                    .map(move |x| (x, y))
            })
            .collect()
    }

    fn coords(&self) -> Vec<(i32, i32)> {
        (self.ymin-1..self.ymax+1)
            .flat_map(|y| (self.xmin-1..self.xmax+1).map(move |x| (x,y)))
            .collect()
    }

    fn debug(&self) {
        println!("map");
        for y in self.ymin-4..self.ymax+4 {
            println!(
                "{}",
                ((self.xmin-4..self.xmax+4)
                    .map(|x| self.val((x,y)))
                    .map(|v| if v { '#' } else { '.' })
                    .collect::<String>())
            )
        }
    }

    fn val(&self, pt: (i32, i32)) -> bool {
        if (self.xmin..self.xmax).contains(&pt.0) && (self.ymin..self.ymax).contains(&pt.1) {
            self.arr.contains(&pt)
        } else {
            self.bg
        }
    }

    fn coeff(&self, pt: (i32, i32)) -> usize {
        return self.neigh(pt)
            .iter()
            .map(|pt| self.val(*pt))
            .fold(0usize, |val, bits| {
                val << 1 | if bits { 1 } else { 0 }
            })
    }
}

#[aoc_generator(day20)]
pub fn generator(input: &str) -> Map {
    let (codec, map) = input.split_once("\n\n").unwrap();

    let codec: HashMap<usize, bool> = codec
        .chars()
        .take(512)
        .enumerate()
        .map(|(n, c)| if c == '#' { (n, true) } else { (n, false) })
        .collect();

    let (cols, lines) = (map.lines().next().unwrap().len(), map.lines().count());
    let map: HashSet<(i32, i32)> = map
        .lines()
        .enumerate()
        .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .filter_map(|(x,y,c)| if c == '#' { Some((x as i32,y as i32)) } else { None })
        .collect();

    Map {
        xmax: cols as i32,
        xmin: 0,
        ymax: lines as i32,
        ymin: 0,
        codec,
        arr: map,
        bg: false
    }
}

#[aoc(day20, part1)]
pub fn part1(input: &Map) -> usize {
    let mut image = input.clone();
    image.debug();
    println!("{}", image.coeff((1,2)));
    println!("{}", image.codec.get(&image.coeff((1,2))).unwrap());
    for _i in 0..2 {
        let transform = image
            .coords()
            .iter()
            .filter_map(|pt| {
                let val = image.codec.get(&image.coeff(*pt)).unwrap();
                if *val {
                    Some(*pt)
                } else {
                    None
                }
            })
            .collect();

        image.arr = transform;
        image.grow();
        if !image.bg {
            image.bg = *image.codec.get(&0).unwrap();
        } else {
            image.bg = *image.codec.get(&511).unwrap();
        }

        image.debug();
    }

    image.arr.iter().count()
}

