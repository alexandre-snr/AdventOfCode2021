use std::fs;

fn part_one(lines: &Vec<&str>, line_length: i32) {
    let mut gamma_rate: String = String::from("");
    let mut epsilon_rate: String = String::from("");
    for i in 0..line_length {
        let mut ones = 0;
        for num in lines.iter() {
            if num.chars().nth(i as usize).unwrap() == '1' {
                ones += 1;
            }
        }

        if ones >= lines.len() as i32 / 2 {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        } else {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        }
    }

    let gamma_rate = i32::from_str_radix(&gamma_rate[..], 2).unwrap();
    let epsilon_rate = i32::from_str_radix(&epsilon_rate[..], 2).unwrap();
    println!("result: {}", gamma_rate * epsilon_rate);
}

fn get_winners(lines: &Vec<&str>, f: fn(char, char) -> bool) -> String {
    let mut bit_index = 0;
    let mut winners = lines.clone();

    while winners.len() > 1 {
        let mut ones = 0;
        for num in winners.iter() {
            if num.chars().nth(bit_index as usize).unwrap() == '1' {
                ones += 1;
            }
        }
        let most_common_bit = match ones >= winners.len() as i32 - ones {
            true => '1',
            false => '0',
        };

        winners = winners
            .clone()
            .iter()
            .filter(|x| f(x.chars().nth(bit_index).unwrap(), most_common_bit))
            .map(|x| x.clone())
            .collect();
        bit_index += 1;
    }
    String::from(winners[0])
}

fn get_o2(lines: &Vec<&str>) -> String {
    get_winners(lines, |x, y| x == y)
}

fn get_co2(lines: &Vec<&str>) -> String {
    get_winners(lines, |x, y| x != y)
}

fn part_two(lines: &Vec<&str>) {
    let o2 = get_o2(&lines);
    let o2 = i32::from_str_radix(&o2[..], 2).unwrap();
    let co2 = get_co2(&lines);
    let co2 = i32::from_str_radix(&co2[..], 2).unwrap();

    println!("winner: {} * {} = {}", o2, co2, o2 * co2);
}

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("could not read input file");

    let lines: Vec<&str> = file_content.lines().collect();
    let line_length: i32 = lines.get(0).expect("no lines").len() as i32;

    part_one(&lines, line_length);
    part_two(&lines);
}
