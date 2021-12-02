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
            Direction::Down => position.1 += ele.1,
            Direction::Forward => position.0 += ele.1,
            Direction::Up => position.1 -= ele.1,
        }
    }

    println!("{:?} = {}", position, position.0 * position.1);
}

pub fn simple_sol1() {
    let output = INPUT.lines().map(|x| x.split(' ')).fold((0, 0), |acc, mut item| {
        let x = item.next().unwrap().chars().next().unwrap();
        let y = item.next().unwrap().parse::<i32>().unwrap();
        match x {
            'd' => (acc.0, acc.1 + y),
            'f' => (acc.0 + y, acc.1),
            'u' => (acc.0, acc.1 - y),
            _ => unreachable!(),
        }
    });
    println!("{}", output.0 * output.1);
}

pub fn sol2() {
    let mut position = (0, 0, 0);

    for ele in parse_input() {
        match ele.0 {
            Direction::Down => position.0 += ele.1,
            Direction::Up => position.0 -= ele.1,
            Direction::Forward => {
                position.1 += ele.1; 
                position.2 += ele.1 * position.0;
            },
        }
    }

    println!("{:?} = {}", position, position.1 * position.2);
}

pub fn simple_sol2() {
    let output = INPUT.lines().map(|x| x.split(' ')).fold((0, 0, 0), |acc, mut item| {
        let x = item.next().unwrap().chars().next().unwrap();
        let y = item.next().unwrap().parse::<i32>().unwrap();
        match x {
            'd' => (acc.0 + y, acc.1, acc.2),
            'f' => (acc.0, acc.1 + y, acc.2 + (acc.0 * y)),
            'u' => (acc.0 - y, acc.1, acc.2),
            _ => unreachable!(),
        }
    });
    println!("{}", output.1 * output.2);
}
