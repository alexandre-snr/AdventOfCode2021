use std::fs;

fn part_one() {
    const OPENERS: &str = "[({<";
    const CLOSERS: &str = "])}>";
    const SCORE: [u32; 4] = [57, 3, 1197, 25137];

    let file_content = fs::read_to_string("input.txt").expect("could not read input file");

    let lines = file_content.lines();
    let mut stack = Vec::new();
    let mut score = 0;

    for line in lines {
        for c in line.chars() {
            if OPENERS.contains(c) {
                stack.push(c);
            } else if CLOSERS.contains(c) {
                let opener = stack.last().unwrap().clone();
                let opener_index = OPENERS.chars().position(|x| x == opener).unwrap();
                let expected_closer = CLOSERS.chars().nth(opener_index).unwrap();

                if c != expected_closer {
                    let score_index = CLOSERS.chars().position(|x| x == c).unwrap();
                    score += SCORE[score_index];
                    break;
                }
                stack.pop();
            }
        }
    }
    println!("{}", score);
}

fn part_two() {
    const OPENERS: &str = "[({<";
    const CLOSERS: &str = "])}>";
    const SCORE: [u32; 4] = [2, 1, 3, 4];

    let file_content = fs::read_to_string("input.txt").expect("could not read input file");

    let lines = file_content.lines();
    let mut correct_lines = Vec::new();

    for line in lines {
        let mut valid = true;
        let mut stack = Vec::new();

        for c in line.chars() {
            if OPENERS.contains(c) {
                stack.push(c);
            } else if CLOSERS.contains(c) {
                let opener = stack.last().unwrap().clone();
                let opener_index = OPENERS.chars().position(|x| x == opener).unwrap();
                let expected_closer = CLOSERS.chars().nth(opener_index).unwrap();

                if c != expected_closer {
                    valid = false;
                    break;
                }
                stack.pop();
            }
        }
        if valid {
            correct_lines.push(stack);
        }
    }

    let mut scores: Vec<u128> = Vec::new();
    for line in correct_lines {
        let mut score: u128 = 0;
        for c in line.into_iter().rev() {
            let index = OPENERS.chars().position(|x| x == c).unwrap();
            score *= 5;
            score += SCORE[index] as u128;
        }
        scores.push(score);
    }

    scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = scores.len() / 2;
    println!("{}", scores[mid]);
}

fn main() {
    part_one();
    part_two();
}
