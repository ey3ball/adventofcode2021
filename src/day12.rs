use std::collections::HashMap;
type Caves = HashMap<String, Vec<String>>;

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Caves {
    let mut caves: Caves = HashMap::new();
    caves.insert("end".to_owned(), vec![]);
    input
        .lines()
        .for_each(|l| {
            let (from, to) = l.split_once("-").unwrap();
            let (from, to) = (from.to_owned(), to.to_owned());
            caves.entry(from.clone()).or_insert(vec![]).push(to.clone());
            if to != "end" && from != "start" {
                caves.entry(to).or_insert(vec![]).push(from);
            }
        });
    caves
}

pub fn explore(input: &Caves, twice: bool) -> Vec<Vec<String>> {
    let mut paths: Vec<(bool, Vec<String>)> = vec![(false, vec!["start".to_owned()])];

    loop {
        let mut next_paths: Vec<(bool, Vec<String>)> = vec![];
        paths
            .iter()
            .for_each(|(detour_taken, path)| {
                let last = path.iter().last().unwrap();
                if last == "end" {
                    next_paths.push((*detour_taken, path.clone()));
                } else {
                    let options = &input[last];
                    options
                        .iter()
                        .filter(|next| next == &&next.to_ascii_uppercase() || !path.contains(next) || !detour_taken)
                        .filter(|next| next != &"start")
                        .for_each(|valid_next| {
                            let mut new_path = path.clone();
                            let detour = if new_path.contains(valid_next) && &valid_next.to_ascii_uppercase() != valid_next {
                                true
                            } else {
                                *detour_taken
                            };
                            new_path.push(valid_next.clone());
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
pub fn part1(input: &Caves) -> usize {
    let paths = explore(input, false);
    paths.len()
}

#[aoc(day12, part2)]
pub fn part2(input: &Caves) -> usize {
    let paths = explore(input, true);
    paths.len()
}
