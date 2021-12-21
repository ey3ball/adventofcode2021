use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Clone)]
pub struct Scanner {
    beacons: Vec<Pos>,
    base: Option<Pos>,
    rot: Option<usize>,
}

#[aoc_generator(day19)]
pub fn parse(input: &str) -> Vec<Scanner> {
    let mut scanners: Vec<_> = input
        .split("\n\n")
        .map(|beacons| Scanner {
            base: None,
            rot: None,
            beacons: beacons
                .lines()
                .skip(1)
                .map(|coords| {
                    let v: Vec<_> = coords.split(',').map(|c| c.parse().unwrap()).collect();
                    Pos {
                        x: v[0],
                        y: v[1],
                        z: v[2],
                    }
                })
                .collect(),
        })
        .collect();

    // scanner 0 is the reference for all others
    scanners[0].base = Some(Pos { x: 0, y: 0, z: 0 });
    scanners[0].rot = Some(0);
    scanners
}

impl Pos {
    fn rev_rots(&self) -> impl Iterator<Item = Pos> + '_ {
        // Generate coordinates for all possible scanner alignemnts
        //
        [
            [self.x, self.y, self.z],
            [self.x, -self.z, self.y],
            [self.x, -self.y, -self.z],
            [self.x, self.z, -self.y],
            [self.z, self.x, self.y],
            [self.y, self.x, -self.z],
            [-self.z, self.x, -self.y],
            [-self.y, self.x, self.z],
            [self.y, self.z, self.x],
            [-self.z, self.y, self.x],
            [-self.y, -self.z, self.x],
            [self.z, -self.y, self.x],
            [-self.x, self.z, self.y],
            [-self.x, self.y, -self.z],
            [-self.x, -self.z, -self.y],
            [-self.x, -self.y, self.z],
            [self.y, -self.x, self.z],
            [-self.z, -self.x, self.y],
            [-self.y, -self.x, -self.z],
            [self.z, -self.x, -self.y],
            [self.z, self.y, -self.x],
            [self.y, -self.z, -self.x],
            [-self.z, -self.y, -self.x],
            [-self.y, self.z, -self.x],
        ]
        .into_iter()
        .map(|xyz| Pos {
            x: xyz[0],
            y: xyz[1],
            z: xyz[2],
        })
    }

    fn rots(&self) -> impl Iterator<Item = Pos> + '_ {
        // Generate coordinates for all possible scanner alignemnts
        //
        [
            [self.x, self.y, self.z],
            [self.x, self.z, -self.y],
            [self.x, -self.y, -self.z],
            [self.x, -self.z, self.y],
            [self.y, self.z, self.x],
            [self.y, self.x, -self.z],
            [self.y, -self.z, -self.x],
            [self.y, -self.x, self.z],
            [self.z, self.x, self.y],
            [self.z, self.y, -self.x],
            [self.z, -self.x, -self.y],
            [self.z, -self.y, self.x],
            [-self.x, self.z, self.y],
            [-self.x, self.y, -self.z],
            [-self.x, -self.z, -self.y],
            [-self.x, -self.y, self.z],
            [-self.y, self.x, self.z],
            [-self.y, self.z, -self.x],
            [-self.y, -self.x, -self.z],
            [-self.y, -self.z, self.x],
            [-self.z, self.y, self.x],
            [-self.z, self.x, -self.y],
            [-self.z, -self.y, -self.x],
            [-self.z, -self.x, self.y],
        ]
        .into_iter()
        .map(|xyz| Pos {
            x: xyz[0],
            y: xyz[1],
            z: xyz[2],
        })
    }

    fn rot(&self, n: usize) -> Option<Pos> {
        self.rots().nth(n)
    }

    fn rev_rot(&self, n: usize) -> Option<Pos> {
        self.rev_rots().nth(n)
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
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Scanner {
    pub fn most_likely(&self, other: &Scanner) -> Option<(usize, Pos)> {
        let mut matches: Vec<(usize, usize, usize, usize, Pos)> = self
            .pairings(other)
            .iter()
            .map(|(which_from, which_to, which_rot, s2_s1)| {
                (
                    other
                        .rebase(*which_rot, *s2_s1)
                        .iter()
                        .filter(|ob| self.beacons.contains(&ob))
                        .count(),
                    *which_from,
                    *which_to,
                    *which_rot,
                    *s2_s1,
                )
            })
            .filter(|(c, _, _, _, _)| *c >= 12)
            .collect();
        matches.sort();

        for m in matches.iter() {
            println!("=:{} b1:{} b2:{} rot:{} d:{:?}", m.0, m.1, m.2, m.3, m.4);
        }

        if matches.len() > 0 {
            let m = matches[0];
            println!("=:{} b1:{} b2:{} rot:{}", m.0, m.1, m.2, m.3);
            Some((m.3, m.4))
        } else {
            None
        }
    }

    /* Generate all possible coordinates pairings between both scanners
     * (each beacon of self paired with each beacon with other for all possible rotations),
     * then return the corresponding base change vector */
    pub fn pairings(&self, other: &Scanner) -> Vec<(usize, usize, usize, Pos)> {
        self.beacons
            .iter()
            .enumerate()
            .flat_map(|(from_idx, from_b)| {
                other
                    .beacons
                    .iter()
                    .enumerate()
                    .flat_map(move |(to_idx, to_b)| {
                        to_b.rev_rots().enumerate().map(move |(rot_idx, to_b_rot)| {
                            (from_idx, to_idx, rot_idx, to_b_rot - *from_b)
                        })
                    })
            })
            .collect()
    }

    pub fn rebase(&self, rot: usize, rel: Pos) -> Vec<Pos> {
        self.beacons
            .iter()
            .map(|pos| pos.rev_rot(rot).unwrap() - rel)
            .collect()
    }
}

#[aoc(day19, part1)]
pub fn part1(input: &Vec<Scanner>) -> usize {
    let mut scanners = input.clone();
    let found: Vec<Scanner> = vec![];
    let process = vec![scanners[0].clone()];
    let mut others: Vec<Scanner> = scanners[1..].iter().cloned().collect();

    //while !others.is_empty() {
    //    for p in process.iter() {
    //        others
    //            .iter_mut()
    //            .filter_map(|o| p.most_likely(o).map(|r| (o,r)))
    //            ;
    //    }
    //}

    let (rot, rel) = input[0].most_likely(&input[1]).unwrap();
    println!("{:?}", input[1].beacons);
    println!("{:?}", rel.rot(rot).unwrap());
    println!("{:?}", Pos { x: 0, y: 0, z: 0 } - rel);
    let new_scan1 = Scanner {
        beacons: input[1].rebase(rot, rel),
        base: None,
        rot: None,
    };
    println!("===");
    println!("{:?}", new_scan1.beacons);

    let (rot2, rel2) = input[4].most_likely(&new_scan1).unwrap();
    let new_scan4 = input[4].rebase(rot2, rel2);
    println!("{:?}", input[4]);
    println!("{:?}", rel2);
    println!("{:?}", rel2.rot(rot2));
    println!("{:?}", Pos { x: 0, y: 0, z: 0 } - rel2);
    println!("---4---");
    println!("{:?}", new_scan4);

    0
}
