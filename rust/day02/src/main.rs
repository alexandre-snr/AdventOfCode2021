use std::fs;

enum CommandType {
    Forward,
    Up,
    Down,
}

struct Command {
    command_type: CommandType,
    distance: i32,
}

fn part_one(commands: &Vec<Command>) {
    let mut horizontal_pos: i32 = 0;
    let mut depth: i32 = 0;
    for command in commands {
        match command.command_type {
            CommandType::Forward => horizontal_pos += command.distance,
            CommandType::Up => depth -= command.distance,
            CommandType::Down => depth += command.distance,
        }
    }

    let result = horizontal_pos * depth;
    println!("{} * {} = {}", horizontal_pos, depth, result);
}

fn part_two(commands: &Vec<Command>) {
    let mut horizontal_pos: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;
    for command in commands {
        match command.command_type {
            CommandType::Forward => {
                horizontal_pos += command.distance;
                depth += aim * command.distance;
            },
            CommandType::Up => aim -= command.distance,
            CommandType::Down => aim += command.distance,
        }
    }

    let result = horizontal_pos * depth;
    println!("{} * {} = {}", horizontal_pos, depth, result);
}

fn main() {
    let commands: Vec<Command> = fs::read_to_string("input.txt")
        .expect("could not read input file")
        .lines()
        .map(|x| {
            let mut split = x.split(" ");
            Command {
                command_type: match split.next().unwrap() {
                    "forward" => CommandType::Forward,
                    "up" => CommandType::Up,
                    "down" => CommandType::Down,
                    &_ => panic!("unknown command"),
                },
                distance: split.next().unwrap().parse::<i32>().unwrap(),
            }
        })
        .collect();

    part_one(&commands);
    part_two(&commands);
}
