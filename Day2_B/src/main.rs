// A - Rock, B - Paper, C - Scissors
// X - Lose, Y - Draw, Z - Win
// 1pt for Rock, 2pt for Paper, 3pt for Scissors + 0pt if lose, 3pt if draw, 6pt if win
// Rock > Scissors, Paper > Rock, Scissors > Paper
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input").unwrap();
    let mut score: i32 = 0;

    for line in file_contents.lines() {
        // I need to lose
        if line.chars().nth(2).unwrap() == 'X' {
            if line.chars().nth(0).unwrap() == 'A' {
                score = score + 3; // show scissors against rock
            }
            else if line.chars().nth(0).unwrap() == 'B' {
                score = score + 1; // show rock against paper
            }
            else if line.chars().nth(0).unwrap() == 'C' {
                score = score + 2; // show paper against scissors
            }
        }
        // I need to draw
        else if line.chars().nth(2).unwrap() == 'Y' {
            score = score + 3;
            if line.chars().nth(0).unwrap() == 'A' {
                score = score + 1; // show rock against rock
            }
            else if line.chars().nth(0).unwrap() == 'B' {
                score = score + 2; // show paper against paper
            }
            else if line.chars().nth(0).unwrap() == 'C' {
                score = score + 3; // show scissors against scissors
            }
        }
        // I need to win
        else if line.chars().nth(2).unwrap() == 'Z' {
            score = score + 6;
            if line.chars().nth(0).unwrap() == 'A' {
                score = score + 2; // show paper against rock
            }
            else if line.chars().nth(0).unwrap() == 'B' {
                score = score + 3; // show scissors against paper
            }
            else if line.chars().nth(0).unwrap() == 'C' {
                score = score + 1; // show rock against scissors
            }
        }
    }
    println!("{}", score);
}
