use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("vituiks mään");
    let input: Vec<i32> = input
        .trim()
        .split("\n")
        .map(|s| match s.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        })
        .collect();
    part1(input.clone());
    part2(input.clone());
}

fn part1(input: Vec<i32>) {
    let mut counter = -1;
    let mut previous = 0;
    for i in input {
        if i > previous {
            counter = counter + 1;
        }
        previous = i;
    }
    println!("part 1 answer: {}", counter);
}

fn part2(input: Vec<i32>) {
    let mut i = 0;
    let mut j = 3;
    let mut counter = 0;
    while j < input.len() {
        if input.get(i) < input.get(j) {
            counter = counter + 1;
        }
        i = i + 1;
        j = j + 1;
    }
    println!("part 2 answer: {}", counter);
}
