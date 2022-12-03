use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input").unwrap();
    let mut max_calories: [i32; 3] = [0; 3];
    let mut sum: i32 = 0;
    
    for line in file_contents.lines() {
        if line.trim().is_empty() {
            if sum > max_calories[0]{
                max_calories[2] = max_calories[1];
                max_calories[1] = max_calories[0];
                max_calories[0] = sum;
            }
            else if sum > max_calories[1]{
                max_calories[2] = max_calories[1];
                max_calories[1] = sum;
            }
            else if sum > max_calories[2]{
                max_calories[2] = sum;
            }
            sum = 0;
        }
        else {
            sum = sum + line.parse::<i32>().unwrap();
        }
    }
    println!("{}", max_calories[2] + max_calories[1] + max_calories[0]);
}
