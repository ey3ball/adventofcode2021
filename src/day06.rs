#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<u32> {
    input.split(',').map(|f| f.parse().unwrap()).collect()
}

#[aoc(day6, part1)]
pub fn part1(fish: &[u32]) -> usize {
    let mut fish = fish.to_owned();

    for _day in 1..=80 {
        let mut ready = 0;
        for f in fish.iter_mut() {
            if *f == 0 {
                ready += 1;
                *f = 6
            } else {
                *f -= 1;
            }
        }
        for _new in 0..ready {
            fish.push(8);
        }
        println!("day: {} {}", _day, fish.len());
    }
    fish.len()
}

#[aoc(day6, part2)]
pub fn part2(fish: &[u32]) -> usize {
    let mut fish = fish.to_owned();
    let mut history: Vec<usize> = vec![fish.len()];

    for _day in 1..=10 {
        let mut ready = 0;
        for f in fish.iter_mut() {
            if *f == 0 {
                ready += 1;
                *f = 6
            } else {
                *f -= 1;
            }
        }
        for _new in 0..ready {
            fish.push(8);
        }
        history.push(fish.len());
    }

    let mut count = history[10];
    for day in 11..=256 {
        let ready = (history[day - 7] - history[day - 8]) + (history[day - 9] - history[day - 10]);

        count += ready;
        history.push(count);
    }
    history.pop().unwrap()
}
