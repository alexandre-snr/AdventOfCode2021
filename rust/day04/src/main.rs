use std::fmt;
use std::fs;

use colored::*;

#[derive(Clone, PartialEq)]
struct Board {
    data: [u32; 25],
    called: [bool; 25],
}

impl Board {
    fn call(&mut self, number: u32) {
        for i in 0..25 {
            if self.data[i] == number {
                self.called[i] = true;
            }
        }
    }

    fn is_line_winning(&self, y: i32) -> bool {
        for x in 0..5 {
            let i = y * 5 + x;
            if !self.called[i as usize] {
                return false;
            }
        }
        return true;
    }

    fn is_col_winning(&self, x: i32) -> bool {
        for y in 0..5 {
            let i = y * 5 + x;
            if !self.called[i as usize] {
                return false;
            }
        }
        return true;
    }

    fn is_winning(&self) -> bool {
        for i in 0..5 {
            if self.is_line_winning(i) || self.is_col_winning(i) {
                return true;
            }
        }
        false
    }

    fn calc_score(&self) -> u32 {
        let mut sum = 0;

        for i in 0..25 {
            if !self.called[i] {
                sum += self.data[i];
            }
        }
        sum
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..5 {
            for x in 0..5 {
                let i = y * 5 + x;
                let mut text = self.data[i].to_string().white();
                if self.called[i] {
                    text = text.bold();
                }
                write!(f, "{0: <4}", text).expect("could not write");
            }
            write!(f, "\n").expect("could not write");
        }
        Ok(())
    }
}

fn parse_board(lines: &mut std::iter::Peekable<std::str::Lines>) -> Board {
    let mut data: [u32; 25] = [0; 25];
    let called: [bool; 25] = [false; 25];

    for y in 0..5 {
        let line = lines.next().unwrap();
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().expect("invalid number in board"))
            .collect();
        for x in 0..5 {
            data[y * 5 + x] = numbers[x];
        }
    }
    lines.next();

    Board {
        data: data,
        called: called,
    }
}

fn get_winning_board(boards: &Vec<Board>) -> Option<&Board> {
    for board in boards {
        if board.is_winning() {
            return Some(board)
        }
    }
    None
}

fn call(boards: &mut Vec<Board>, number: u32) {
    for board in boards {
        board.call(number);
    }
}

fn part_one(file_content: &String) {
    let mut lines = file_content.lines().peekable();

    let first_line = lines.next().unwrap();
    let mut numbers = first_line
        .split(',')
        .map(|x| x.parse::<u32>().expect("invalid number in first line"));
    lines.next();

    let mut boards: Vec<Board> = Vec::new();
    while lines.peek() != None {
        boards.push(parse_board(&mut lines))
    }

    let mut last_called = 0;
    while get_winning_board(&boards).is_none() {
        let n = numbers.next().unwrap();
        call(&mut boards, n);
        last_called = n;
    }

    println!("{}", get_winning_board(&boards).unwrap().calc_score() * last_called);
}

fn part_two(file_content: &String) {
    let mut lines = file_content.lines().peekable();

    let first_line = lines.next().unwrap();
    let mut numbers = first_line
        .split(',')
        .map(|x| x.parse::<u32>().expect("invalid number in first line"));
    lines.next();

    let mut boards: Vec<Board> = Vec::new();
    while lines.peek() != None {
        boards.push(parse_board(&mut lines))
    }

    let mut last_called = 0;
    let mut last_winning: Option<Board> = None;
    loop {
        let n = match numbers.next() {
            Some(n) => n,
            None => { break },
        };
        call(&mut boards, n);

        let current_win = get_winning_board(&boards);
        if current_win.is_some() {
            last_winning = Some(current_win.unwrap().clone());
            last_called = n;

            let index = boards.iter().position(|x| x == last_winning.as_ref().unwrap()).unwrap();
            boards.remove(index);
        }
    }

    println!("{}", last_winning.unwrap().calc_score() * last_called);
}

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("could not read input file");
    part_one(&file_content);
    part_two(&file_content);
}
