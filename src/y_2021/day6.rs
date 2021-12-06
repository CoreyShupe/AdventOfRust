use std::collections::HashMap;

const INPUT: &str = include_str!("../../input/2021/day6.txt");

pub fn guess_growth(days_left: i32, memoized_input: &mut HashMap<i32, u64>) -> u64 {
    if let Some(memoized_response) = memoized_input.get(&days_left) {
        return *memoized_response;
    }

    let x = (6..=days_left)
        .step_by(7)
        .map(|step| guess_growth(days_left - step - 3, memoized_input))
        .fold(1, |total, current| total + current);

    memoized_input.insert(days_left, x);

    x
}

pub fn get_days_response(days: i32) {
    let mut memoized = HashMap::new();

    let response = INPUT
        .split(',')
        .map(|initial_timer| days + (5 - initial_timer.parse::<i32>().unwrap()))
        .map(|days_left| guess_growth(days_left, &mut memoized))
        .sum::<u64>();

    println!("For days {}, {} fish are made.", days, response);
}

pub fn sol1() {
    get_days_response(80);
}

pub fn sol2() {
    get_days_response(256);
}
