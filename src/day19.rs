#[derive(Debug, Copy)]
pub struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
pub struct Scanner {
    beacons: Vec<Pos>,
}

#[aoc_generator(day19)]
pub fn parse(input: &str) -> Vec<Scanner> {
    input
        .split("\n\n")
        .map(|beacons| Scanner {
            beacons: beacons
                .lines()
                .skip(1)
                .map(|coords| {
                    let v: Vec<_> = coords.split(',').map(|c| c.parse().unwrap()).collect();
                    Pos { x: v[0], y: v[1], z: v[2] }
                })
                .collect(),
        })
        .collect()
}

impl Pos {
    fn rots(&self) -> Vec<Pos> {
        // Generate coordinates for all possible scanner alignemnts
        [
            [self.x, self.y, self.z],
            [self.y, self.z, self.x],
            [self.z, self.x, self.y]
        ]
            .iter()
            .flat_map(|xyz| {
                [
                    [xyz[0],  xyz[1],  xyz[2]],
                    [-xyz[0], xyz[1],  xyz[2]],
                    [xyz[0],  -xyz[1], xyz[2]],
                    [-xyz[0], -xyz[1], xyz[2]],
                    [xyz[0],  xyz[1],  -xyz[2]],
                    [-xyz[0], xyz[1],  -xyz[2]],
                    [xyz[0],  -xyz[1], -xyz[2]],
                    [-xyz[0], -xyz[1], -xyz[2]]
                ]
            })
            .map(|xyz| Pos {x: xyz[0], y: xyz[1], z: xyz[2]})
            .collect()
    }
}

impl std::ops::Sub for Pos {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


impl Scanner {
    pub fn most_likely(&self, other: &Scanner) {
        rebases(otherà


    }

    pub fn rebases(&self, other: &Scanner) -> Vec<Pos> {
        self.beacons
            .iter()
            .flat_map(|from_b| other.beacons.iter().map(|to_b| to_b - from_b))
            .collect()
    }
}

#[aoc(day19, part1)]
pub fn part1(input: &Vec<Scanner>) -> usize {
    println!("{:?}", input);
    println!("{:?}", input.len());
    0
}

