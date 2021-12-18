use std::fs;

const S: usize = 10;

#[derive(Copy, Clone)]
struct Octopus {
    energy: u8,
    has_flashed: bool,
}

type Map = [[Octopus; S]; S];

fn flash(map: &mut Map, x: usize, y: usize) -> u32 {
    let mut flashes = 1;
    map[x][y].energy = 0;
    map[x][y].has_flashed = true;

    let mut x_off: i32 = -1;
    let mut y_off: i32 = -1;
    while x_off <= 1 {
        while y_off <= 1 {
            let final_x = x as i32 + x_off;
            let final_y = y as i32 + y_off;

            if x_off == 0 && y_off == 0 {
                y_off += 1;
                continue;
            }
            if final_x < 0 || final_x >= S as i32 || final_y < 0 || final_y >= S as i32 {
                y_off += 1;
                continue;
            }

            if map[final_x as usize][final_y as usize].has_flashed {
                y_off += 1;
                continue;
            }

            map[final_x as usize][final_y as usize].energy += 1;
            if map[final_x as usize][final_y as usize].energy >= 10 {
                flashes += flash(map, final_x as usize, final_y as usize);
            }

            y_off += 1;
        }
        x_off += 1;
        y_off = -1;
    }

    flashes
}

fn step(map: &mut Map) -> u32 {
    let mut flashes = 0;
    for x in 0..S {
        for y in 0..S {
            map[x][y].energy += 1;
            map[x][y].has_flashed = false;
        }
    }

    for x in 0..S {
        for y in 0..S {
            if map[x][y].energy >= 10 && !map[x][y].has_flashed {
                flashes += flash(map, x, y);
            }
        }
    }

    flashes
}

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("could not read input file");

    let mut map: Map = [[Octopus {
        energy: 0,
        has_flashed: false,
    }; S]; S];
    let mut x = 0;
    let mut y = 0;
    for line in file_content.lines() {
        for num in line.chars() {
            map[x][y].energy = num.to_digit(10).unwrap() as u8;
            x += 1;
        }

        x = 0;
        y += 1;
    }

    let mut flashes = 0;

    for i in 0..10000 {
        let step_flashes = step(&mut map);
        flashes += step_flashes;


        if i == 99 {
            println!("part one: {}", flashes);
        }

        if step_flashes == ((S * S) as u32).try_into().unwrap() {
            println!("part two: {}", i + 1);
            break;
        }
    }

}
