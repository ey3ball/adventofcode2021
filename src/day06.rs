#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<u32> {
    input
        .split(",")
        .map(|f| f.parse().unwrap())
        .collect()
}

#[aoc(day6, part1)]
pub fn part1(fish: &Vec<u32>) -> usize {
    let mut fish = fish.clone(); 

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
pub fn part2(fish: &Vec<u32>) -> usize {
    let mut fish = fish.clone(); 
    let mut history: Vec<usize> = vec![];
    history.push(fish.len());

    for day in 1..=256 {
        //println!("day {:?}", fish);
        let mut ready = 0;
        for f in fish.iter_mut() {
            if *f == 0 {
                ready += 1;
                *f = 6
            } else {
                *f -= 1;
            }
        }

        if day > 10 {
            ready += history[day - 7] - history[day - 8];
        }

        for _new in 0..ready {
            fish.push(8);
        }

        let count_old = if day <= 10 {
            fish.iter().filter(|&&c| c < 7).count()
        } else {
            history[day - 10] + fish.iter().filter(|&&c| c < 7).count()
        };
        let count_new = fish.iter().filter(|&&c| c >= 7).count();
        history.push(count_old + count_new);
        if day == 10 {
            fish = fish[history[0]..].iter().copied().collect();
        } else if day > 10 {
            fish = fish.iter().skip_while(|&&c| c == 6).copied().collect();
        } else {
            println!("day {}: {}", day, count_old + count_new);
        }
        println!("day {}: {}", day, count_old + count_new);
        println!("{}", fish.len());
    }
    history.pop().unwrap()
}
