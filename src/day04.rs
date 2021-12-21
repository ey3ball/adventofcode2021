#[derive(Clone)]
pub struct Card {
    rows: usize,
    cols: usize,
    grid: Vec<u32>,
    check: Vec<bool>,
    win: Option<usize>,
}

type Bingo = (Vec<u32>, Vec<Card>);

impl Card {
    fn new(grid: Vec<u32>, rows: usize, cols: usize) -> Card {
        Card {
            rows,
            cols,
            check: vec![false; grid.len()],
            grid,
            win: None,
        }
    }

    fn coords(&self, index: usize) -> (usize, usize) {
        let col = index % self.cols;
        (col, (index - col) / self.rows)
    }

    fn row(&self, index: usize) -> Vec<usize> {
        let (_, row) = self.coords(index);
        (0..self.cols).map(|c| row * self.cols + c).collect()
    }

    fn col(&self, index: usize) -> Vec<usize> {
        let (col, _) = self.coords(index);
        (0..self.rows).map(|r| r * self.cols + col).collect()
    }

    fn filled(&self, indexes: Vec<usize>) -> bool {
        indexes.iter().all(|&i| self.check[i])
    }

    fn bingo(&mut self, number: u32) -> bool {
        let pos = self.grid.iter().position(|&item| item == number);
        match pos {
            Some(index) => {
                self.check[index] = true;
                if self.filled(self.col(index)) || self.filled(self.row(index)) {
                    self.win = Some(index);
                }
                self.win.is_some()
            }
            None => false,
        }
    }
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Bingo {
    let mut lines = input.lines();
    let draw = lines
        .next()
        .unwrap()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();
    lines.next();

    let cols = lines.clone().next().unwrap().split_whitespace().count();
    let rows = lines.clone().take_while(|&r| !r.is_empty()).count();

    let cards: Vec<Card> = lines
        .filter(|&r| !r.is_empty())
        .collect::<Vec<&str>>()
        .chunks(rows)
        .map(|card| {
            card.iter()
                .flat_map(|row| row.split_whitespace().map(|n| n.parse().unwrap()))
                .collect::<Vec<u32>>()
        })
        .map(|grid| Card::new(grid, rows, cols))
        .collect();

    (draw, cards)
}

#[aoc(day4, part1)]
pub fn part1((draw, cards): &Bingo) -> u32 {
    let mut cards = cards.clone();
    let mut bag = draw.iter();
    let mut has_winner = false;
    let mut drawn = 0;
    while !has_winner {
        drawn = *bag.next().unwrap();
        has_winner = cards.iter_mut().any(|c| c.bingo(drawn));
    }

    let winner = cards.iter().find(|c| c.win.is_some()).unwrap();
    winner
        .grid
        .iter()
        .enumerate()
        .filter(|&(i, _)| !winner.check[i])
        .map(|(_, v)| v)
        .sum::<u32>()
        * drawn
}

#[aoc(day4, part2)]
pub fn part2((draw, cards): &Bingo) -> u32 {
    let mut cards = cards.clone();
    let mut bag = draw.iter();
    let mut drawn = 0;

    while cards.len() != 1 {
        drawn = *bag.next().unwrap();
        cards.iter_mut().for_each(|c| {
            c.bingo(drawn);
        });
        cards.retain(|c| c.win.is_none())
    }

    while cards[0].win.is_none() {
        drawn = *bag.next().unwrap();
        cards[0].bingo(drawn);
    }
    cards[0]
        .grid
        .iter()
        .enumerate()
        .filter(|&(i, _)| !cards[0].check[i])
        .map(|(_, v)| v)
        .sum::<u32>()
        * drawn
}
