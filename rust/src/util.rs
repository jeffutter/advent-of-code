pub fn read_input(day: usize) -> String {
    std::fs::read_to_string(format!("../inputs/day{:0>2}", day)).unwrap()
}
