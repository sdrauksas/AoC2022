use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input").unwrap();
    let mut max_calories: i32 = 0;
    let mut sum: i32 = 0;
    
    for line in file_contents.lines() {
        if line.trim().is_empty() {
            if sum > max_calories{
                max_calories = sum;
            }
            sum = 0;
        }
        else {
            sum = sum + line.parse::<i32>().unwrap();
        }
    }
    println!("{}", max_calories);
}
