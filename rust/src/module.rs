#[macro_export]
macro_rules! generate_main {
    ($($mod_name:ident)*) => {
        use util;
        use std::time::{Duration, Instant};

        fn measure_time<T, F: Fn() -> T>(func: F) -> (T, Duration) {
            let start = Instant::now();
            let res = func();
            let duration = start.elapsed();
            (res, duration)
        }

        $(
            use $mod_name;
        )*

        fn main() {

            $(
              let day_s = stringify!($mod_name).trim_start_matches("day");
              let day = usize::from_str_radix(day_s, 10).unwrap();

              let (res, duration) = measure_time(|| $mod_name::part1(util::read_input("..", day)));
              println!("Day{:0>2}-01 {: >8}μs:\t{}", day, duration.as_micros(), res);

              let (res, duration) = measure_time(|| $mod_name::part2(util::read_input("..", day)));
              println!("Day{:0>2}-02 {: >8}μs:\t{}", day, duration.as_micros(), res);
            )*
        }
    };
}
