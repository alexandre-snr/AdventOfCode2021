use std::collections::HashMap;
use std::fs;

type Fishs = HashMap<i8, u64>;

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("could not read file");
    let nums = file_content
        .lines()
        .next()
        .expect("no line")
        .split(',')
        .map(|x| x.parse::<i8>().expect("invalid number"));

    let mut fishs: Fishs = HashMap::new();
    for i in -1..9 {
        fishs.insert(i, 0);
    }
    for num in nums {
        fishs.entry(num).and_modify(|x| *x += 1);
    }

    for _ in 0..256 {
        for i in 0..9 {
            *fishs.get_mut(&(i - 1)).unwrap() = *fishs.get(&i).unwrap();
        }

        let new_borns =*fishs.get(&-1).unwrap();
        *fishs.get_mut(&6).unwrap() += new_borns;
        *fishs.get_mut(&8).unwrap() = new_borns;
    }

    let mut sum = 0;
    for i in 0..9 {
        sum += *fishs.get(&i).unwrap();
    }
    println!("result: {}", sum);
}
