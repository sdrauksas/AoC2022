use std::{fs, iter::Enumerate, collections::linked_list::IterMut};

fn main() {
    let file_contents = fs::read_to_string("input").unwrap();
    let mut group: [String; 3] = ["".to_string(), "".to_string(), "".to_string()];
    let mut count = 0;
    let mut priorities: i32 = 0;

    for line in file_contents.lines() {
        // Split into groups
        group[count] = line.to_string();
        count = count + 1;
        // Group is filled up
        if count == 3 {
            let mut already_checked = "".to_string();
            // Iterate over all items of the first rucksac
            for badge_one in group[0].chars() {
                if already_checked.find(badge_one) == None { // Was this item already searched?
                    let badge_two = group[1].find(badge_one); // Find the same item in the second rucksac
                    if badge_two != None { // Found one!
                        let badge_three = group[2].find(badge_one); // Find the same item in the third rucksac
                        if badge_three != None { // Found one
                            if badge_one.is_lowercase() { // Convert ASCII to priority
                                priorities = priorities + (badge_one as i32 - '`' as i32);
                            }
                            else if badge_one.is_uppercase() {
                                priorities = priorities + (badge_one as i32 - '@' as i32 + 26);
                            }
                        }
                    }
                    already_checked = already_checked + &badge_one.to_string(); // Don't look for the same item again
                }
            }
            count = 0;
            group = ["".to_string(), "".to_string(), "".to_string()];
        }
    }
    println!("{}", priorities);
}
