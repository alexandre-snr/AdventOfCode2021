use std::fs;

fn part_one(lines: &Vec<i32>) {
    let mut iter = lines.iter();
    let mut last_number = iter.next().unwrap();
    let mut increases = 0;

    for number in lines {
        if number > last_number {
            increases += 1;
        }
        last_number = &number
    }

    println!("increases count: {}", increases);
}

fn get_moving_average(lines: &Vec<i32>, index: usize) -> i32 {
    let values: &[i32] = &lines[index..index + 3];
    values.iter().sum()
}

fn part_two(lines: &Vec<i32>) {
    let mut increases = 0;

    for i in 0..lines.len() - 3 {
        let sum_a = get_moving_average(&lines, i);
        let sum_b = get_moving_average(&lines, i + 1);

        if sum_b > sum_a {
            increases += 1;
        }
    }

    println!("increases count: {}", increases);
}

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("could not read file");
    let lines: Vec<i32> = file_content
        .lines()
        .map(|x| x.parse::<i32>().expect("one value is not a number"))
        .collect();

    part_one(&lines);
    part_two(&lines);
}
