use std::collections::HashMap;

const INPUT: &str = include_str!("../../input/2021/day6.txt");

pub fn guess_growth(
    days_left: i32,
    timer_start: i32,
    inner: i32,
    memoized_input: &mut HashMap<i32, u64>,
) -> u64 {
    if memoized_input.contains_key(&days_left) {
        return *memoized_input.get(&days_left).unwrap();
    }
    // println!("Layer {}: FISH STEP {}, {}", inner, 18 - days_left, timer_start);
    let mut x = 0;
    for days_step in (timer_start..=days_left).step_by((timer_start + 1) as usize) {
        // println!("Grown fish {} -> {}", days_step, days_left - days_step);
        // println!("{}: STEP {}", inner + 1, days_step);
        if days_left - days_step <= 8 {
            x += 1;
        } else {
            x += guess_growth((days_left - days_step) - 3, 6, inner + 1, memoized_input);
        }
    }
    memoized_input.insert(days_left, x + 1);
    x + 1
}

pub fn sol1() {
    let numbers = INPUT
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    let days = 80;

    let mut join_handles = Vec::new();

    for y in numbers {
        join_handles.push(std::thread::spawn(move || {
            guess_growth(days + (5 - y), 6, 0, &mut HashMap::new())
        }));
    }

    let mut x = 0;
    for ele in join_handles {
        x += ele.join().unwrap();
    }
    dbg!(x);
}

pub fn sol2() {
    let numbers = INPUT
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    let days = 256;

    let mut join_handles = Vec::new();

    for y in numbers {
        join_handles.push(std::thread::spawn(move || {
            guess_growth(days + (5 - y), 6, 0, &mut HashMap::new())
        }));
    }

    let mut x = 0;
    for ele in join_handles {
        x += ele.join().unwrap();
    }
    dbg!(x);
}
