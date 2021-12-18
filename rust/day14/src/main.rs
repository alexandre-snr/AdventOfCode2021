use std::collections::HashMap;
use std::fs;

fn step(template: &String, rules: &HashMap<String, char>) -> String {
    let mut subs: Vec<String> = Vec::new();
    for i in 0..template.len() - 1 {
        subs.push(String::from(&template[i..i + 2]));
    }

    let mut new = String::new();
    for sub in subs {
        let rule = rules.get(&String::from(&sub));
        new.push(sub.chars().nth(0).unwrap());
        if rule.is_some() {
            new.push(*rule.unwrap());
        }
    }
    new.push(template.chars().nth_back(0).unwrap());
    new
}

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("could not read input");
    let mut lines = file_content.lines();

    let mut template = String::from(lines.next().expect("initial template"));
    lines.next();

    let mut rules: HashMap<String, char> = HashMap::new();
    for rule in lines {
        let normalized_rule = rule.replace(" -> ", "");
        let mut chars = normalized_rule.chars();
        rules.insert(String::from(&normalized_rule[0..2]), chars.nth(2).unwrap());
    }
    for i in 0..40 {
        template = step(&template, &rules);
        println!("{}", i);
    }

    let mut counts: HashMap<char, u128> = HashMap::new();
    for c in template.chars() {
        match counts.get_mut(&c) {
            Some(v) => *v += 1,
            None => {
                counts.insert(c, 1);
            }
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
