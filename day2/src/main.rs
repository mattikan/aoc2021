use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("vituiks mään");
    let vec: Vec<(&str, i32)> = input.trim().lines()
    .map(|s| {
        let mut f = s.split(" ");
        (f.next().unwrap(),
        f.next().unwrap().parse::<i32>().unwrap())
    }).collect();
    part1(vec.clone());
    part2(vec.clone());
}

fn part1(input: Vec<(&str, i32)>) {
    let mut pos = 0;
    let mut dep = 0;
    for (command, value) in input {
        match command {
            "forward" => pos = pos + value,
            "up" => dep = dep - value,
            "down" => dep = dep + value,
            _ => println!("wtf")
        }
    }
    println!("part 1 result: pos {}, dep {}, answer {}", pos, dep, pos*dep);
}

fn part2(input: Vec<(&str, i32)>) {
    let mut pos = 0;
    let mut dep = 0;
    let mut aim = 0;
    for (command, value) in input {
        match command {
            "forward" => {
                pos = pos + value;
                dep = dep + (aim*value)
            }
            "up" => aim = aim - value,
            "down" => aim = aim + value,
            _ => println!("wtf")
        }
    }
    println!("part 2 result: pos {}, dep {}, answer {}", pos, dep, pos*dep);
}