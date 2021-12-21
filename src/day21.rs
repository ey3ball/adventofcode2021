//const START_POSITIONS: (i64, i64) = (4, 8);
const START_POSITIONS: (i64, i64) = (1, 2);
use itertools::Itertools;

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
