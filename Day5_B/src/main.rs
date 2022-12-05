use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input").unwrap();
    let mut stacks_input = "".to_string();
    let mut commands_input = "".to_string();
    

    // Split input into crate stacks and commands
    for line in file_contents.lines() {
        let mut result = line.find("[");
        if result != None {
            stacks_input = stacks_input + line;
            stacks_input.push('\n');
            continue;
        }
        result = line.find("move");
        if result != None {
            commands_input = commands_input + line;
            commands_input.push('\n');
        }
    }

    // Read stacks into vectors
    let mut stacks: [Vec<char>; 9] = Default::default();
    for line in stacks_input.lines().rev() {
        for i in 0..9 {
            let crate_id = line.chars().nth(i*4+1).unwrap();
            stacks[i].push(crate_id);
        }
    }
    for i in 0..9 {
        let mut temp = stacks[i].pop().unwrap();
        while temp == ' ' {
            temp = stacks[i].pop().unwrap();
        }
        stacks[i].push(temp);
    }

    // Execute commands
    for line in commands_input.lines() {
        let commands: Vec<&str> = line.split_terminator(" ").collect();
        let from = commands[3].parse::<usize>().unwrap() - 1;
        let to = commands[5].parse::<usize>().unwrap() - 1;
        let mut moving: Vec<char> = vec![];
        for _i in 0..commands[1].parse().unwrap() {
            let temp = stacks[from].pop().unwrap();
            moving.push(temp);
        }
        for _i in 0..moving.len() {
            stacks[to].push(moving.pop().unwrap());
        }
    }

    print!("{}", stacks[0].pop().unwrap());
    print!("{}", stacks[1].pop().unwrap());
    print!("{}", stacks[2].pop().unwrap());
    print!("{}", stacks[3].pop().unwrap());
    print!("{}", stacks[4].pop().unwrap());
    print!("{}", stacks[5].pop().unwrap());
    print!("{}", stacks[6].pop().unwrap());
    print!("{}", stacks[7].pop().unwrap());
    println!("{}", stacks[8].pop().unwrap());
}