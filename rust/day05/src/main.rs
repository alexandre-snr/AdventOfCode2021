use std::fmt;
use std::fs;
use std::cmp::Ordering;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({};{})", self.x, self.y).expect("could not write");
        Ok(())
    }
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn get_step(&self) -> Point {Point {
            x: match self.end.x.cmp(&self.start.x) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1
            },
            y: match self.end.y.cmp(&self.start.y) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1
            },
        }
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.start, self.end).expect("could not write");
        Ok(())
    }
}

fn parse_line(line: &String) -> Line {
    let splits: Vec<Vec<i32>> = line
        .split("->")
        .map(|x| {
            x.trim()
                .split(",")
                .map(|y| y.parse::<i32>().expect("invalid number"))
                .collect()
        })
        .collect();
    Line {
        start: Point {
            x: splits[0][0],
            y: splits[0][1],
        },
        end: Point {
            x: splits[1][0],
            y: splits[1][1],
        },
    }
}

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("could not read input file");
    let lines = file_content.lines().map(|x| parse_line(&String::from(x)));

    let mut board: [u32; 1000 * 1000] = [0; 1000 * 1000];

    for line in lines {
        // uncomment for part one
        // if line.start.x != line.end.x && line.start.y != line.end.y {
        //     continue;
        // }

        let mut pos = Point {
            x: line.start.x,
            y: line.start.y,
        };
        let step = line.get_step();
        while pos.x != line.end.x || pos.y != line.end.y {
            board[(pos.y * 1000 + pos.x) as usize] += 1 as u32;
            pos.x += step.x;
            pos.y += step.y;
        }
        board[(pos.y * 1000 + pos.x) as usize] += 1 as u32;
    }

    let mut sum = 0;
    for i in 0..1000*1000 {
        if board[i] >= 2 {
            sum += 1;
        }
    }

    println!("result: {}", sum);
}
