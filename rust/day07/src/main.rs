use std::fs;

fn part_one(fuel: i32) -> i32 {
    fuel
}

#[allow(dead_code)]
fn part_two(fuel: i32) -> i32 {
    let mut sum = 0;
    for i in 0..fuel+1 {
        sum += i;
    }
    sum
}

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("could not read input file");

    let nums = file_content
        .split(',')
        .map(|x| x.parse::<i32>().expect("invalid number"));

    let mut min = 1000;
    let mut max = 0;
    for num in nums.clone() {
        if num > max {
            max = num;
        } else if num < min {
            min = num;
        }
    }

    let mut min_fuel: i32 = i32::MAX;
    for target in min..max+1 {
        let mut fuel = 0;
        for num in nums.clone() {
            fuel += part_one((target - num).abs());
            // fuel += part_two((target - num).abs());
        }

        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    println!("min_fuel {}", min_fuel);
}
