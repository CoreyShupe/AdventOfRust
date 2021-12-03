const INPUT: &str = include_str!("../../input/2021/day3.txt");

pub fn sol1() {
    let count = INPUT.lines().count();
    let out = INPUT.lines().map(|x| x.split("")).fold(Vec::new(), |mut accum, split| {
        split.enumerate().for_each(|(idx, value)| {
            if value.is_empty() {
                return;
            }
            if accum.len() <= (idx - 1) {
                accum.push(0);
            }
            if value == "1" {
                accum[idx - 1] += 1;
            }
        });
        accum
    });
    let mut bin1 = String::from("");
    let mut bin2 = String::from("");
    for x in out {
        if x > (count/2) {
            bin1.push('1');
            bin2.push('0');
        } else {
            bin1.push('0');
            bin2.push('1');
        }
    }
    println!("{}", usize::from_str_radix(&bin1, 2).unwrap() * usize::from_str_radix(&bin2, 2).unwrap());
}

pub fn simple_sol1() {
    let count = INPUT.lines().count() / 2;

    let bin_str = INPUT
        .lines()
        .map(|x| x.chars().enumerate())
        .flatten()
        .fold(vec![0;INPUT.lines().next().unwrap().len()], |mut result, (idx, x)| {result[idx] += (x == '1') as usize; result})
        .into_iter()
        .fold(String::new(), |mut bin_str, item| {bin_str.push(((('0' as u32) + ((item > count) as u32)) as u8) as char); bin_str});

    let bin = usize::from_str_radix(&bin_str, 2).unwrap();
    println!("{}", bin * (!bin & 0b111111111111));
}

pub fn sol2() {
    fn reduce(input: Vec<&str>, idx: usize, big: char, small: char) -> Vec<&str> {
        if input.len() == 1 {
            return input;
        }

        let count = input.iter().filter(|x| x.chars().nth(idx).unwrap() == '1').count();
        reduce(if count > input.len() - count || count == input.len() - count {
            input.into_iter().filter(|x| x.chars().nth(idx).unwrap() == big).collect()
        } else {
            input.into_iter().filter(|x| x.chars().nth(idx).unwrap() == small).collect()
        }, idx + 1, big, small)
    }

    let r1 = reduce(INPUT.lines().collect(), 0, '1', '0');
    let r2 = reduce(INPUT.lines().collect(), 0, '0', '1');

    println!("{}", usize::from_str_radix(r1[0], 2).unwrap() * usize::from_str_radix(r2[0], 2).unwrap());
}
