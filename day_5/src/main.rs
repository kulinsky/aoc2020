use std::fs::read_to_string;

fn input_to_usize(s: &str) -> usize {
    let seat_id: String = s.chars()
        .map(|x| match x {
            'F' => '0',
            'B' => '1',
            'L' => '0',
            'R' => '1',
            _ => x
        }).collect();
    usize::from_str_radix(&seat_id, 2).unwrap()
}

fn find_seat(data: &Vec<usize>) -> Result<usize, ()> {
    for x in *data.into_iter().min().unwrap()..*data.into_iter().max().unwrap() {
        if !data.contains(&x) && data.contains(&(x - 1)) && data.contains(&(x + 1)) {
            return Ok(x)
        }
    }
    Err(())
}

fn main() {
    let data: Vec<usize> = read_to_string("src/input.txt")
        .unwrap().lines().map(|x| input_to_usize(x)).collect();
    println!("Part 1: {}", data.iter().max().unwrap());
    println!("Part 2: {}", find_seat(&data).unwrap());
}
