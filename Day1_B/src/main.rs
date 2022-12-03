use std::fs;

fn main() {
    let file_path = "input";
    let file_contents_full = fs::read_to_string(file_path).unwrap();
    let file_contents_lines = file_contents_full.lines();
    let mut max_calories: [i32; 3] = [0; 3];
    let mut sum: i32 = 0;
    
    for line in file_contents_lines {
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
