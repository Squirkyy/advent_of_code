pub fn part1() {
    let s = String::from(include_str!("puzzle.txt"));
    let parts = s.split("\n");
    let mut value = 0;
    for part in parts {
        let test: Vec<_> = part.chars().filter(|c| c.is_ascii_digit()).collect();
        let mut string = String::from(test[0]);
        string.push(test[test.len() - 1]);
        value += string.parse::<i32>().unwrap();
    }
    println!("The sum of all of the calibration values is {}.", value);
}

pub fn part2() {
    let s = String::from(include_str!("puzzle.txt"));
    let parts = s.split("\n");

    // Mapping of spelled-out numbers to their corresponding digits
    let number_map = [
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut value = 0;
    for part in parts {
        let mut modified_part = part.to_string();

        // Replace spelled-out numbers with digits
        for (word, digit) in number_map.iter() {
            modified_part = modified_part.replace(word, digit);
        }

        let test: Vec<_> = modified_part
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect();
        if test.len() >= 2 {
            let mut string = String::from(test[0]);
            string.push(test[test.len() - 1]);
            value += string.parse::<i32>().unwrap();
        }
    }
    println!("The sum of all of the calibration values is {}.", value);
}
