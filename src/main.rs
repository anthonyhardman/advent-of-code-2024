mod day1;
mod day2;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    day: u32,
}

fn main() {
    let args = Args::parse();
    
    match args.day {
        1 => day1::solution(),
        2 => day2::solution(),
        26.. => println!("Advent of Code only runs for 25 days"),
        _ => println!("Day not implemented"),
    }
}
