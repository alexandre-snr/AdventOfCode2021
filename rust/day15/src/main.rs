use std::fs;

const S: usize = 10;
type Map = [[u8; S]; S];
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn display_map(map: &Map) {
    for y in 0..S {
        for x in 0..S {
            print!("{}", map[x][y]);
        }
        println!();
    }
}

fn get_adjacents(point: &Point, forbidden_points: &Vec<Point>) -> Vec<Point> {
    let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut adjacents: Vec<Point> = Vec::new();
    'outer: for delta in deltas {
        let adjacent = Point {
            x: point.x + delta.0,
            y: point.y + delta.1,
        };
        for forbidden_point in forbidden_points {
            if adjacent.x == forbidden_point.x && adjacent.y == forbidden_point.y {
                continue 'outer;
            }
        }
        if adjacent.x >= 0 && adjacent.x < S as i32 && adjacent.y >= 0 && adjacent.y < S as i32 {
            adjacents.push(adjacent);
        }
    }
    adjacents
}

fn lowest_score_to_bottom_right(
    map: &Map,
    starting_point: &Point,
    forbidden_points: Vec<Point>,
) -> u32 {
    if starting_point.x == S as i32 - 1 && starting_point.y == S as i32 - 1 {
        return map[starting_point.x as usize][starting_point.y as usize] as u32;
    }
    let adjacents = get_adjacents(starting_point, &forbidden_points);
    let mut lowest_score = 10000000;

    let mut next_forbidden_points = forbidden_points.clone();
    next_forbidden_points.push(starting_point.clone());
    for adjacent in adjacents {
        let score = lowest_score_to_bottom_right(&map, &adjacent, next_forbidden_points.clone());
        if score < lowest_score {
            lowest_score = score
        }
    }
    lowest_score += map[starting_point.x as usize][starting_point.y as usize] as u32;
    lowest_score
}

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("read input file");
    let lines = file_content.lines();
    let mut map: Map = [[0; S]; S];

    for (line, y) in lines.zip(0..S) {
        for (c, x) in line.chars().zip(0..S) {
            map[x][y] = c.to_digit(10).unwrap() as u8;
        }
    }

    display_map(&map);
    println!(
        "res: {}",
        lowest_score_to_bottom_right(&map, &Point { x: 0, y: 0 }, Vec::new())
    )
}
