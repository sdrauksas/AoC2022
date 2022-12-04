use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input").unwrap();
    let mut count: i32 = 0;

    for line in file_contents.lines() {
        let elves: Vec<&str> = line.split_terminator(",").collect();
        
        let f_sections: Vec<&str> = elves[0].split_terminator("-").collect();
        let f_start = f_sections[0].parse::<i32>().unwrap();
        let f_end = f_sections[1].parse::<i32>().unwrap();

        let s_sections: Vec<&str> = elves[1].split_terminator("-").collect();
        let s_start = s_sections[0].parse::<i32>().unwrap();
        let s_end = s_sections[1].parse::<i32>().unwrap();


        if s_start >= f_start && s_start <= f_end {
            // If the second section's start is within the first section's range
            count = count + 1; 
        }
        else if f_start >= s_start && f_start <= s_end {
            // If the first section's start is within the second section's range
            count = count + 1; 
        }
    }
    println!("{}", count);
}