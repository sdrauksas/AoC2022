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

        if f_start == s_start {
            // If the section starts are equal, then one of them will be fully
            // overlapping the other
            count = count + 1;
        }
        else if f_start < s_start && f_end >= s_start && f_end >= s_end {
            // If the first section's start is smaller than the second section's
            // start and the second's start is smaller than the first's end,
            // the sections will overlap only if the ends are equal or the first
            // is bigger
            count = count + 1;
        }
        else if f_start > s_start && f_start <= s_end && f_end <= s_end {
            // If the first section's start is bigger than the second section's
            // start and the first's start is smaller than the seconds's end,
            // the sections will overlap only if the ends are equal or the
            // second is bigger
            count = count + 1;
        }
    }
    println!("{}", count);
}