use std::fs;

struct Point {
    x: i32,
    y: i32,
}

enum Axis {
    Horizontal,
    Vertical,
}

struct Instruction {
    axis: Axis,
    position: i32,
}

fn display_map(points: &Vec<Point>) {
    let mut max_x = 0;
    let mut max_y = 0;

    for point in points {
        if max_x < point.x {
            max_x = point.x
        }
        if max_y < point.y {
            max_y = point.y
        }
    }

    for y in 0..max_y+1 {
        for x in 0..max_x+1 {
            let mut filled = false;
            for point in points {
                if point.x == x && point.y == y {
                    filled = true;
                    break;
                }
            }

            if filled {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn fold_horizontal(points: &mut Vec<Point>, instr: &Instruction) {
    for point in points.iter_mut() {
        if point.y > instr.position {
            point.y = instr.position + (instr.position - point.y);
        }
    }
}

fn fold_vertical(points: &mut Vec<Point>, instr: &Instruction) {
    for point in points.iter_mut() {
        if point.x > instr.position {
            point.x = instr.position + (instr.position - point.x);
        }
    }
}

fn count_uniques(points: &Vec<Point>) -> usize {
    let mut uniques: Vec<Point> = Vec::new();

    'outer: for point in points {
        for unique in uniques.iter() {
            if point.x == unique.x && point.y == unique.y {
                continue 'outer;
            }
        }
        uniques.push(Point {
            x: point.x,
            y: point.y
        })
    }
    uniques.len()
}

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("could not read input file");
    let mut points: Vec<Point> = Vec::new();
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut has_passed_nl = false;

    for line in file_content.lines() {
        if line.len() == 0 {
            has_passed_nl = true;
            continue;
        }

        if !has_passed_nl {
            let split: Vec<i32> = line
                .split(',')
                .map(|x| x.parse::<i32>().expect("valid number"))
                .collect();
            points.push(Point {
                x: split.get(0).expect("x pos").clone(),
                y: split.get(1).expect("y pos").clone(),
            })
        } else {
            let split: Vec<&str> = line.split_whitespace().collect();
            let split: Vec<&str> = split.get(2).expect("three words").split('=').collect();

            instructions.push(Instruction {
                axis: match *split.get(0).expect("axis") {
                    "x" => Axis::Vertical,
                    "y" => Axis::Horizontal,
                    _ => panic!("invalid axis"),
                },
                position: split
                    .get(1)
                    .expect("position")
                    .parse::<i32>()
                    .expect("valid position"),
            });
        }
    }

    let mut iter = instructions.iter();
    let first_instr = iter.next().expect("has one instruction");
    match first_instr.axis {
        Axis::Horizontal => fold_horizontal(&mut points, &first_instr),
        Axis::Vertical => fold_vertical(&mut points, &first_instr),
    }
    println!("count uniques {}", count_uniques(&points));

    for instr in iter {
        match instr.axis {
            Axis::Horizontal => fold_horizontal(&mut points, &instr),
            Axis::Vertical => fold_vertical(&mut points, &instr),
        }
    }
    display_map(&points);
}
