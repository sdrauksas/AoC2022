use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input").unwrap();
    let mut priorities: i32 = 0;

    for line in file_contents.lines() {
        let (first, second) = line.split_at(line.len()/2);
        let mut already_found = "".to_string();

        for item in first.chars() { // Iterate through first compartment
            let wrong_compartment = second.find(item);
            
            if wrong_compartment != None && already_found.find(item) == None {
                if item.is_lowercase() { // Convert ASCII to priority
                    priorities = priorities + (item as i32 - '`' as i32);
                }
                else if item.is_uppercase() {
                    priorities = priorities + (item as i32 - '@' as i32 + 26);
                }
                already_found = already_found + &item.to_string();
            }
        }
    }
    println!("{}", priorities);
}
