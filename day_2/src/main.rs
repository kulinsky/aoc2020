use std::fs::read_to_string;

struct Password {
    low: usize,
    high: usize,
    letter: char,
    password: String,
}

impl Password {
    fn is_valid_part_one(&self) -> bool {
        (self.low..=self.high).contains(&self.password.matches(self.letter).count())
    }
    fn is_valid_part_two(&self) -> bool {
        (self.password.chars().nth(self.low - 1).unwrap() == self.letter) !=
            (self.password.chars().nth(self.high - 1).unwrap() == self.letter)
    }
}

fn parse_password(line: &String) -> Password {
    let parted = line.split(&[' ', '-', ':'][..])
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();
    Password {
        low: parted[0].parse::<usize>().unwrap(),
        high: parted[1].parse::<usize>().unwrap(),
        letter: parted[2].chars().next().unwrap(),
        password: String::from(parted[3]),
    }
}

fn main() {
    let data: Vec<String> = read_to_string("src/input.txt")
        .unwrap().lines().map(|s| s.parse().unwrap()).collect();
    println!("Part1: {}", data.iter()
        .map(|x| Password::is_valid_part_one(&parse_password(x)))
        .filter(|y| *y).count());
    println!("Part2: {}", data.iter()
        .map(|x| Password::is_valid_part_two(&parse_password(x)))
        .filter(|y| *y).count());
}
