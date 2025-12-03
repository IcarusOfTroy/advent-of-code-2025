use std::fs::File;
use std::io::*;
use std::path::Path;

fn main() {
    let input_file = Path::new("input.txt");
    let path = input_file.display();

    let line_file_reader = BufReader::new(File::open(&input_file).expect(&format!("Cannot open {}", path)));

    let mut dial = 50;
    let mut zero_count = 0;

    for line in line_file_reader.lines() {
        print!("Current dial position: {} ", dial);
        let instruction = parse_instruction(&line.unwrap());
        print!("Instruction: {}{}, ", instruction.0, instruction.1);
        dial = turn_dial(dial, &instruction.0, instruction.1);
        if dial == 0 {
            zero_count += 1;
        }
        println!("New dial position: {}", dial);
    }
    println!("Zero count: {}", zero_count);
}

fn parse_instruction(instruction: &str) -> (String, i32) {
    let direction = &instruction[0..1];
    let steps: i32 = instruction[1..].trim().parse().unwrap();
    (direction.to_string(), steps)
}

fn turn_dial(current_position: i32, direction: &str, steps: i32) -> i32 {
    let sanitized_steps = steps % 100;
    let new_position = match direction {
        "L" => {
            let next = current_position - sanitized_steps;
            if next < 0 {
                100 + next
            } else {
                next
            }
       }
        "R" => {
            let next = current_position + sanitized_steps;
            if next > 99 {
                next - 100
            } else {
                next
            }
        }
        _ => current_position,
    };
    new_position
}
