use std::collections::HashMap;
type Caves<'a> = HashMap<&'a str, Vec<&'a str>>;

pub fn parse(input: &str) -> Caves {
    let mut caves: Caves = HashMap::new();
    caves.insert("end", vec![]);
    input.lines().for_each(|l| {
        let (from, to) = l.split_once("-").unwrap();
        caves.entry(from).or_insert_with(Vec::new).push(to);
        if to != "end" && from != "start" {
            caves.entry(to).or_insert_with(Vec::new).push(from);
        }
    });
    caves
}

pub fn explore<'a>(input: &'a Caves, twice: bool) -> Vec<Vec<&'a str>> {
    let mut paths: Vec<(bool, Vec<&str>)> = vec![(!twice, vec!["start"])];

    loop {
        let mut next_paths: Vec<(bool, Vec<&str>)> = vec![];
        paths.iter().for_each(|(detour_taken, path)| {
            let last = path.iter().last().unwrap();
            if last == &"end" {
                next_paths.push((*detour_taken, path.clone()));
            } else {
                let options = &input[last];
                options
                    .iter()
                    .filter(|next| {
                        next == &&next.to_ascii_uppercase() || !path.contains(next) || !detour_taken
                    })
                    .filter(|next| next != &&"start")
                    .for_each(|valid_next| {
                        let mut new_path = path.clone();
                        let detour = if new_path.contains(valid_next)
                            && &valid_next.to_ascii_uppercase() != valid_next
                        {
                            true
                        } else {
                            *detour_taken
                        };
                        new_path.push(*valid_next);
                        next_paths.push((detour, new_path));
                    });
            }
        });

        if paths == next_paths {
            break;
        } else {
            //println!("{:?}", paths);
            //println!("{:?}", next_paths);
            paths = next_paths;
        }
    }
    paths.into_iter().map(|(_, path)| path).collect()
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    let caves = parse(input);
    let paths = explore(&caves, false);
    paths.len()
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
    let caves = parse(input);
    let paths = explore(&caves, true);
    paths.len()
}
