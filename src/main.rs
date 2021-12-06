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

solve!(y_2021, day6);
