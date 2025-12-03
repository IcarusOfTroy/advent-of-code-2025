use std::fs::File;
use std::io::*;
use std::path::Path;

fn main() {
    let input_file = Path::new("input.txt");
    let path = input_file.display();

    let line_file_reader = BufReader::new(File::open(&input_file).expect(&format!("Cannot open {}", path)));

    let mut dial = 50;
    let mut times_zero_is_clicked = 0;

    for line in line_file_reader.lines() {
        print!("Current dial position: {} ", dial);
        let instruction: (String, i32) = parse_instruction(&line.unwrap());
        
        let step = match instruction.0.as_str() {
            "L" => -(instruction.1),
            "R" => instruction.1,
            _ => 0,
        };

        let result_of_instruction = turn_dial_one_click_at_a_time_counting_zeros(dial, step);
        dial = result_of_instruction.0;
        times_zero_is_clicked += result_of_instruction.1;
        println!("New dial position: {}", dial);
    }
    println!("Zero count: {}", times_zero_is_clicked);
}

fn parse_instruction(instruction: &str) -> (String, i32) {
    let direction = &instruction[0..1];
    let steps: i32 = instruction[1..].trim().parse().unwrap();
    (direction.to_string(), steps)
}

fn turn_dial_one_click_at_a_time_counting_zeros(current_position: i32, steps: i32) -> (i32, i32) {
    let mut position = current_position;
    let mut zero_count = 0;

    let step_direction = if steps > 0 { 1 } else { -1 };
    for _ in 0..steps.abs() {
        position += step_direction;
        if position > 99 {
            position = 0;
        }
        else if position < 0 {
            position = 99;
        }

        if position == 0 {
            zero_count += 1;
        }
    }

    (position, zero_count)
}