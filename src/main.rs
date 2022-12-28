#[macro_use]
extern crate lazy_static;

#[allow(dead_code)]
enum Solution {
    PartOne,
    PartTwo,
}

mod day00;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day08;
mod day09;
mod day10;

fn main() {
    println!("{}", "Hello Advent of Code 2022 (in Rust! 🦀)");
    let mut modify = 0;
    modify_it(&mut modify);
    println!("{}", modify);
}

fn modify_it(m: &mut i32) {
    *m += 5;
}
