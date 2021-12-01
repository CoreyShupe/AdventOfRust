#![allow(unused_macros, dead_code)]

macro_rules! solve {
    ($year:ident, $day:ident $(+ { $($extra:tt)+ })?) => {
        mod $year;
        use $year::$day::*;

        fn main() {
            println!("===  ===  ===  ===  ===  ===  ===");
            println!("Solve for solution 1:");
            sol1();
            println!("===  ===  ===  ===  ===  ===  ===");
            println!("Solve for solution 2:");
            sol2();
            println!("===  ===  ===  ===  ===  ===  ===");
            $(
                $(
                    $extra
                )+
                println!("===  ===  ===  ===  ===  ===  ===");
            )?
        }
    };
}

macro_rules! solve_prone {
    ($year:ident, $day:ident $(+ { $($extra:tt)+ })?) => {
        mod $year;
        use $year::$day::*;

        fn main() -> anyhow::Result<()> {
            println!("Solve for solution 1:");
            println!("===  ===  ===  ===  ===  ===  ===");
            sol1()?;
            println!("===  ===  ===  ===  ===  ===  ===");
            println!("Solve for solution 2:");
            println!("===  ===  ===  ===  ===  ===  ===");
            sol2()?;
            println!("===  ===  ===  ===  ===  ===  ===");
            $(
                $(
                    $extra
                )+
                println!("===  ===  ===  ===  ===  ===  ===");
            )?
            Ok(())
        }
    };
}

// fn main() {
//     println!("{}", r#"
//     let g=|i:&str|{i.lines().flat_map(str::parse).collect::<Vec<u32>>().windows(4).filter(|x|x[0]<x[3]).count()};
//     "#.len())
// }

solve!(
    y_2021,
    day1 + {
        println!("Iter only solution result:");
        iter_only_sol2();
        println!("Optimized:");
        optimized_sol2();
        println!("Golfed:");
        println!("Increased {} times.", golf());
    }
);
