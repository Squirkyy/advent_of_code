trait BagChecker<T> {
    fn is_valid(&self, red_bound: T, blue_bound: T, green_bound: T) -> bool;
}

struct Bag {
    red: i32,
    blue: i32,
    green: i32,
}
impl BagChecker<i32> for Bag {
    fn is_valid(&self, red_bound: i32, blue_bound: i32, green_bound: i32) -> bool {
        return (self.red <= red_bound) && (self.blue <= blue_bound) <= (self.green <= green_bound);
    }
}

pub fn part1() {
    let s = String::from(include_str!("puzzle.txt"));
    let mut sum_valid_games = 0;

    for line in s.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let game_id: i32 = parts[0].replace("Game ", "").parse().unwrap();
        let subsets = parts[1].split("; ");

        let mut game_possible = true;

        for subset in subsets {
            let mut red = 0;
            let mut blue = 0;
            let mut green = 0;

            for cube in subset.split(", ") {
                let details: Vec<&str> = cube.split_whitespace().collect();
                let count: i32 = details[0].parse().unwrap();

                match details[1] {
                    "red" => red += count,
                    "blue" => blue += count,
                    "green" => green += count,
                    _ => (),
                }
            }

            if red > 12 || blue > 14 || green > 13 {
                game_possible = false;
                break;
            }
        }

        if game_possible {
            sum_valid_games += game_id;
        }
    }

    println!("Sum of valid game IDs: {}", sum_valid_games);
}

pub fn part2() {
    let s = String::from(include_str!("puzzle.txt"));
    let mut total_power = 0;

    for line in s.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let subsets = parts[1].split("; ");

        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;

        for subset in subsets {
            let mut red = 0;
            let mut blue = 0;
            let mut green = 0;

            for cube in subset.split(", ") {
                let details: Vec<&str> = cube.split_whitespace().collect();
                let count: i32 = details[0].parse().unwrap();

                match details[1] {
                    "red" => red += count,
                    "blue" => blue += count,
                    "green" => green += count,
                    _ => (),
                }
            }

            max_red = max_red.max(red);
            max_blue = max_blue.max(blue);
            max_green = max_green.max(green);
        }

        // Calculate the power of the minimum set of cubes for this game
        let power = max_red as i64 * max_blue as i64 * max_green as i64;
        total_power += power;
    }

    println!("Sum of the power of minimum sets: {}", total_power);
}
