use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
//mod day22;
mod day23;
mod day24;
mod day25;

fn main() {
    let now = Instant::now();
    day01::solution("input01.txt");
    day02::solution("input02.txt");
    day03::solution("input03.txt");
    day04::solution("input04.txt");
    day05::solution("input05.txt");
    day06::solution("input06.txt");
    day07::solution("input07.txt");
    day08::solution("input08.txt");
    day09::solution("input09.txt");
    day10::solution("input10.txt");
    day11::solution("input11.txt");
    day12::solution("input12.txt");
    day13::solution("input13.txt");
    day14::solution("input14.txt");
    day15::solution("input15.txt");
    day16::solution("input16.txt");
    day17::solution("input17.txt");
    day18::solution("input18.txt");
    day19::solution("input19.txt");
    day20::solution("input20.txt");
    day21::solution("input21.txt");
    //    day22::solution("input22.txt");
    day23::solution("input23.txt");
    day24::solution("input24.txt");
    day25::solution("input25.txt");
    
    let elapsed_time = now.elapsed();
    println!("{} seconds.", elapsed_time.as_secs());
}
