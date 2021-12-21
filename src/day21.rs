use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

//const START_POSITIONS: (i64, i64) = (4, 8);
const START_POSITIONS: (i64, i64) = (1, 2);

pub fn play(dice: &Vec<i64>, pos: &mut i64, score: &mut i64) {
    *pos += dice.iter().sum::<i64>();
    *pos = ((*pos - 1) % 10) + 1;
    *score += *pos;
}

#[aoc(day21, part1)]
pub fn part1(_input: &str) -> i64 {
    let roll = (1i64..=100i64).cycle().enumerate();
    let mut pos = START_POSITIONS;
    let mut scores: (i64, i64) = (0, 0);
    let mut player = 0;

    let mut final_state = (0, 0);
    for draw in &roll.into_iter().chunks(3) {
        let draw: Vec<(usize, i64)> = draw.collect();
        let drawn = draw.iter().map(|(_, die)| *die).collect();
        if player == 0 {
            play(&drawn, &mut pos.0, &mut scores.0);
            player = 1;
            if scores.0 >= 1000 {
                final_state = (draw[2].0 as i64 + 1, scores.1);
                break;
            }
        } else {
            play(&drawn, &mut pos.1, &mut scores.1);
            player = 0;
            if scores.1 >= 1000 {
                final_state = (draw[2].0 as i64 + 1, scores.0);
                break;
            }
        }
        println!("{}: w:{:?} s:{:?}", player, pos, scores);
    }
    println!("{:?}", final_state);
    final_state.0 * final_state.1
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct State {
    scores: (usize, usize),
    player: i8,
}

type Pos = (i64, i64);
type PosPaths = HashMap<Pos, usize>;
type StatePaths = HashMap<State, PosPaths>;

pub fn multiplay(states: &mut StatePaths, cur: &State, rolls: &HashMap<i64, usize>) {
    let from_paths = match states.get(cur) {
        None => return,
        Some(path) => path.clone()
    };

    for (pos, path_count) in from_paths.iter() {
        for (draw, repetitions) in rolls {
            let new_pos = if cur.player == 0 {
                ((pos.0 + *draw - 1) % 10 + 1, pos.1)
            } else {
                (pos.0, (pos.1 + *draw - 1) % 10 + 1)
            };
            let inc_score = if cur.player == 0 {
                (new_pos.0, 0)
            } else {
                (0, new_pos.1)
            };

            let next_state = State {
                scores: (cur.scores.0 + inc_score.0 as usize , cur.scores.1 + inc_score.1 as usize),
                player: (1 + cur.player) % 2
            };

            *states
                .entry(next_state)
                .or_insert(HashMap::new())
                .entry(new_pos)
                .or_insert(0) += repetitions * path_count;
        }
    }
}

#[aoc(day21, part2)]
pub fn part2(_input: &str) -> usize {
    // Compute the frequenties of each possible dice rolls
    let dice_paths = iproduct!(1..=3, 1..=3, 1..=3)
        .map(|(a, b, c)| a + b + c)
        .counts();

    let mut visited: HashSet<State> = HashSet::new();
    let mut states: StatePaths = HashMap::new();
    states.insert(
        State {
            scores: (0, 0),
            player: 0,
        },
        [(START_POSITIONS, 1usize)].iter().copied().collect()
    );

    loop {
        let mut state: Vec<&State> = states.keys().filter(|k| !visited.contains(k)).collect();
        state.sort_by(|s1, s2| (s1.scores.0 + s1.scores.1).partial_cmp(&(s2.scores.0 + s2.scores.1)).unwrap());
        if state.len() == 0 {
            break;
        }
        let state = state[0].clone();

        if state.scores.0 >= 21 || state.scores.1 >= 21 {
            visited.insert(state);
            continue;
        }

        multiplay(&mut states, &state, &dice_paths);

        visited.insert(state);
    }

    let p1_winners: usize =
        states
            .iter()
            .filter(|(k, v)| k.scores.0 >= 21)
            .map(|(_, v)| v.values().sum::<usize>())
            .sum();

    let p2_winners: usize =
        states
            .iter()
            .filter(|(k, v)| k.scores.1 >= 21)
            .map(|(_, v)| v.values().sum::<usize>())
            .sum();

    std::cmp::max(p1_winners, p2_winners)
}
