struct Number {
    start: usize,
    end: usize,
    num: i32,
}

pub fn part1() {
    let s = String::from(include_str!("puzzle.txt"));
    for line in s.lines() {
        println!("{}", line.len());
        let mut isNum = true;
        let mut vec: Vec<i32> = vec![];
        for t in line.chars() {
            if t.is_digit(10) {}
        }
    }
}
