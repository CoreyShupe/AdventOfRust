const INPUT: &str = include_str!("../../input/2021/day2.txt");

fn parse_input() -> Vec<(Direction, i32)> {
    INPUT.lines().map(|item| item.split(' ')).map(|mut items| {
        let direction = match items.next().unwrap() {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => unreachable!(),
        };
        (direction, str::parse(items.next().unwrap()).unwrap())
    }).collect()
}

enum Direction {
    Up,
    Down,
    Forward,
}

pub fn sol1() {
    let mut position = (0, 0);
    
    for ele in parse_input() {
        match ele.0 {
            Direction::Down => position = (position.0, position.1 + ele.1),
            Direction::Forward => position = (position.0 + ele.1, position.1),
            Direction::Up => position = (position.0, position.1 - ele.1),
        }
    }

    println!("{:?} = {}", position, position.0 * position.1);
}

pub fn sol2() {
    let mut position = (0, 0, 0);

    for ele in parse_input() {
        match ele.0 {
            Direction::Down => position = (position.0 + ele.1, position.1, position.2),
            Direction::Up => position = (position.0 - ele.1, position.1, position.2),
            Direction::Forward => position = (position.0, position.1 + ele.1, position.2 + (ele.1 * position.0)),
        }
    }

    println!("{:?} = {}", position, position.1 * position.2);
}