use std::collections::HashMap;
use std::fs;

struct Rule {
    pair_start: char,
    pair_end: char,
    insertion: char,
}

fn step(template: &mut String, rules: &Vec<Rule>) {
    let mut index = 0;

    'main: while index < template.len() - 1 {
        let start = template.chars().nth(index).unwrap();
        let end = template.chars().nth(index + 1).unwrap();
        for rule in rules {
            if rule.pair_start == start && rule.pair_end == end {
                template.insert(index + 1, rule.insertion);
                index += 2;
                continue 'main;
            }
        }
        index += 1;
    }
}

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("could not read input");
    let mut lines = file_content.lines();

    let mut template = String::from(lines.next().expect("initial template"));
    lines.next();

    let mut rules: Vec<Rule> = Vec::new();
    for rule in lines {
        let normalized_rule = rule.replace(" -> ", "");
        let mut chars = normalized_rule.chars();
        rules.push(Rule {
            pair_start: chars.next().expect("first char"),
            pair_end: chars.next().expect("second char"),
            insertion: chars.next().expect("third char"),
        });
    }
    for i in 0..10 {
        println!("{}", i);
        step(&mut template, &rules);
    }

    let mut counts: HashMap<char, i32> = HashMap::new();
    for c in template.chars() {
        match counts.get_mut(&c) {
            Some(v) => *v += 1,
            None => { counts.insert(c, 1); }
        }
    }

    let mut min_val = 10000000;
    let mut max_val = 0;
    for (_, value) in &counts {
        if *value < min_val {
            min_val = *value;
        }
        if *value > max_val {
            max_val = *value;
        }
    }

    println!("{}", max_val - min_val);
}
