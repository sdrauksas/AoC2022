// A - Rock, B - Paper, C - Scissors
// X - Rock, Y - Paper, Z - Scissors
// 1pt for Rock, 2pt for Paper, 3pt for Scissors + 0pt if lose, 3pt if draw, 6pt if win
// Rock > Scissors, Paper > Rock, Scissors > Paper
use std::fs;

fn main() {
    let file_path = "input";
    let file_contents_full = fs::read_to_string(file_path).unwrap();
    let file_contents_lines = file_contents_full.lines();

    let mut score: i32 = 0;

    for line in file_contents_lines {
        // I show rock
        if line.chars().nth(2).unwrap() == 'X' {
            score = score + 1;
            if line.chars().nth(0).unwrap() == 'A' { // Draw
                score = score + 3;
            }
            else if line.chars().nth(0).unwrap() == 'C' { // Win
                score = score + 6;
            }
        }
        // I show paper
        else if line.chars().nth(2).unwrap() == 'Y' {
            score = score + 2;
            if line.chars().nth(0).unwrap() == 'B' { // Draw
                score = score + 3;
            }
            else if line.chars().nth(0).unwrap() == 'A' { // Win
                score = score + 6;
            }
        }
        // I show scissors
        else if line.chars().nth(2).unwrap() == 'Z' {
            score = score + 3;
            if line.chars().nth(0).unwrap() == 'C' { // Draw
                score = score + 3;
            }
            else if line.chars().nth(0).unwrap() == 'B' { // Win
                score = score + 6;
            }
        }
    }
    println!("{}", score);
}
