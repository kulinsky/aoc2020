extern crate typenum;

use std::fs::read_to_string;

use bit_array::BitArray;
use std::time::Instant;


fn part_one(data: &Vec<usize>, data_bitarray: &BitArray<u32, typenum::U2048>) -> Result<usize, ()> {
    for x in data {
        if data_bitarray.get(2020 - x).unwrap() {
            return Ok(x * (2020 - x));
        }
    }
    return Err(());
}

fn part_two(data: &Vec<usize>, data_bitarray: &BitArray<u32, typenum::U2048>) -> Result<usize, ()> {
    for (i, x) in data.iter().enumerate() {
        for y in data.iter().skip(i) {
            if x + y > 2020 { continue; }
            if data_bitarray.get(2020 - x - y).unwrap() {
                return Ok(x * y * (2020 - x - y));
            }
        }
    }
    return Err(());
}

fn main() {
    let now = Instant::now();
    let data: Vec<usize> = read_to_string("src/data.txt")
        .unwrap().lines().map(|s| s.parse().unwrap()).collect();
    let mut data_bitarray = BitArray::<u32, typenum::U2048>::from_elem(false);
    data.iter().for_each(|i| data_bitarray.set(*i, true));
    println!("Part 1: {}", part_one(&data, &data_bitarray).unwrap());
    println!("Part 2: {}", part_two(&data, &data_bitarray).unwrap());
    println!("Elapsed: {}μs", now.elapsed().as_micros());
}
