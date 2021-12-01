#![allow(unused_macros, dead_code)]

macro_rules! solve {
    ($year:ident, $day:ident $(+ { $($extra:tt)+ })?) => {
        mod $year;

        fn main() {
            println!("===  ===  ===  ===  ===  ===  ===");
            println!("Solve for solution 1:");
            $year::$day::sol1();
            println!("===  ===  ===  ===  ===  ===  ===");
            println!("Solve for solution 2:");
            $year::$day::sol2();
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

        fn main() -> anyhow::Result<()> {
            println!("Solve for solution 1:");
            println!("===  ===  ===  ===  ===  ===  ===");
            $year::$day::sol1()?;
            println!("===  ===  ===  ===  ===  ===  ===");
            println!("Solve for solution 2:");
            println!("===  ===  ===  ===  ===  ===  ===");
            $year::$day::sol2()?;
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

solve!(y_2021, day1 + { 
    println!("Iter only solution result:");
    y_2021::day1::iter_only_sol2();
    println!("Optimized:");
    y_2021::day1::optimized_sol2();
});
