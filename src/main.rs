mod day3;

fn main() {
    match day3::part1("src/day3/puzzle.txt") {
        Ok(val) => println!("{}", val),
        Err(er) => panic!("{}", er),
    }
}
