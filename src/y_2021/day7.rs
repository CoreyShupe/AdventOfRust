const INPUT: &str = include_str!("../../input/2021/day7.txt");

pub fn parse_input() -> Vec<i64> {
    INPUT.split(',').map(|x| x.parse().unwrap()).collect()
}

pub fn sol1() {
    fn determine_fuel_usage(vec: &[i64], position: i64) -> i64 {
        vec.iter().map(|x| (x - position).abs()).sum()
    }
    
    let vec = parse_input();
    let max= vec.iter().max().unwrap();
    let mut usage_map = std::collections::HashMap::new();
    dbg!(max);
    for position in 0..*max {
        usage_map.insert(position, determine_fuel_usage(&vec, position));
    }
    dbg!(usage_map.iter().min_by_key(|x| x.1));
}

pub fn sol2() {
    fn addative_factorial(num: i64) -> i64 {
        if num == 0 {
            return 0;
        }
        num + addative_factorial(num - 1)
    }

    fn determine_fuel_usage(vec: &[i64], position: i64) -> i64 {
        vec.iter().map(|x| addative_factorial((x - position).abs())).sum()
    }
    
    let vec = parse_input();
    let max= vec.iter().max().unwrap();
    let mut usage_map = std::collections::HashMap::new();
    dbg!(max);
    for position in 0..*max {
        usage_map.insert(position, determine_fuel_usage(&vec, position));
    }
    dbg!(usage_map.iter().min_by_key(|x| x.1));
}