#[derive(Debug)]
pub struct HeightMap {
    xmax: i32,
    ymax: i32,
    arr: Vec<i8>
}

impl HeightMap {
    fn pt(&self, index: usize) -> (i32, i32) {
        let index = index as i32;
        return (index % self.xmax, (index - (index % self.xmax)) / self.xmax)
    }
    fn idx(&self, pt: (i32, i32)) -> usize {
        return (pt.0 + pt.1 * self.xmax).try_into().unwrap()
    }


    fn neigh<'a>(&'a self, pt: (i32, i32)) -> impl Iterator<Item=(i32, i32)> + 'a {
        (pt.0-1..=pt.0+1)
            .filter(|&x| (0..self.xmax).contains(&x))
            .flat_map(move |x| {
                (pt.1-1..=pt.1+1)
                    .filter(|&y| (0..self.ymax).contains(&y))
                    .map(move |y| (x,y))
            })
            .filter(move |&(x_,y_)| {
                (x_ == pt.0) ^ (y_ == pt.1)
            }) 
    }

    fn neigh_vals<'a>(&'a self, pt: (i32, i32)) -> impl Iterator<Item=i8> + 'a {
        return self.neigh(pt).map(|neigh_pt| self.arr[self.idx(neigh_pt)])
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
    HeightMap { xmax: cols as i32, ymax: lines as i32, arr: map}
}

#[aoc(day9, part1)]
pub fn part1(input: &HeightMap) -> isize {
    input.arr
        .iter()
        .enumerate()
        .filter(|(i, &val)| {
            input.neigh_vals(input.pt(*i))
                 .all(|neigh_val| neigh_val >= val)
        })
        .map(|(_i, val)| (val + 1) as isize)
        .sum()
}

