#[macro_export]
macro_rules! generate_main {
    ($($mod_name:ident)*) => {
        use util;

        $(
            use $mod_name;
        )*

        fn main() {

            $(
              let day_s = stringify!($mod_name).trim_start_matches("day");
              let day = usize::from_str_radix(day_s, 10).unwrap();

              println!("Day{:0>2}-01: {}", day, $mod_name::part1(util::read_input("..", day)));
              println!("Day{:0>2}-02: {}", day, $mod_name::part2(util::read_input("..", day)));
            )*
        }
    };
}
