const INPUT: &str = include_str!("../../input/2021/day1.txt");

fn get_numbers() -> impl Iterator<Item = i32> {
    INPUT
        .lines()
        .map(|item| item.parse::<i32>().expect("Item should be a number"))
}

fn count_increases<I: std::iter::Iterator<Item = i32>>(iter: I) -> i32 {
    iter.fold((0, i32::MAX), |current, item| {
        if item > current.1 {
            (current.0 + 1, item)
        } else {
            (current.0, item)
        }
    })
    .0
}

pub fn sol1() {
    println!("Increases {} times.", count_increases(get_numbers()));
}

pub fn sol2() {
    let mut iter = get_numbers();
    let mut bump_stock = Vec::<i32>::new();
    let mut iter_handle: [[i32; 2]; 2] = [[0, 0], [0, 0]];

    iter_handle[0][0] = iter.next().unwrap();
    iter_handle[0][1] = iter.next().unwrap();
    iter_handle[1][0] = iter_handle[0][1];

    for elem in iter {
        let item = elem;
        iter_handle[1][1] = item;

        bump_stock.push(iter_handle[0][0] + iter_handle[0][1] + item);

        iter_handle[0] = iter_handle[1];
        iter_handle[1] = [item, 0];
    }

    let iter = bump_stock.into_iter();
    println!("Increases {} times.", count_increases(iter));
}

pub fn iter_only_sol2() {
    let increases = count_increases(
        get_numbers()
            .fold(
                (Vec::<i32>::new(), (None, None)),
                |mut accum, item| match accum.1 {
                    (None, None) => (accum.0, (Some(item), None)),
                    (Some(x), None) => (accum.0, (Some(x), Some(item))),
                    (Some(x), Some(y)) => {
                        accum.0.push(x + y + item);
                        (accum.0, (Some(y), Some(item)))
                    }
                    _ => unreachable!(),
                },
            )
            .0
            .into_iter(),
    );
    println!("Increases {} times.", increases);
}

pub fn optimized_sol2() {
    let mut numbers/*: impl std::iter::Iterator<Item = i32>*/ = get_numbers();
    let mut bck = [numbers.next().unwrap(), numbers.next().unwrap(), numbers.next().unwrap()];
    let mut cntr: i32 = 0;

    for x in numbers {
        if bck[0] < x {
            cntr += 1;
        }
        bck[0] = bck[1];
        bck[1] = bck[2];
        bck[2] = x;
    }
    println!("Increases {} times.", cntr);
}

trait Tally<X> {
    fn tally<F: FnMut(&X) -> bool>(self, func: F) -> usize;
}

impl<X, T: Iterator<Item = X>> Tally<X> for T {
    fn tally<F: FnMut(&X) -> bool>(self, func: F) -> usize {
        self.filter(func).count()
    }
}

pub fn golf()->usize{
    let g=|i:Vec<u32>|{ /* golf start */  i.windows(4).tally(|x|x[0]<x[3]) /* golf end */ };
    g(INPUT.lines().flat_map(str::parse).collect())
}
