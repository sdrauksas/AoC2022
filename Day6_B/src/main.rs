use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input").unwrap();
    let mut result = 0;

    // Split input into crate stacks and commands
    for line in file_contents.lines() {
        for i in 14..line.chars().count() {
            let mut fourteen_chars: Vec<char> = line[i-14..i].chars().collect();
            fourteen_chars.sort();
            fourteen_chars.dedup();
            if fourteen_chars.len() == 14 {
                result = i;
                break;
            }
        }
    }

    println!("{}", result);
}
