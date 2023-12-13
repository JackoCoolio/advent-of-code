fn load_file() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn main() {
    let input = load_file();

    let solution: u32 = input
        .lines()
        .map(|line| {
            let mut iter = line.chars().filter_map(|c| c.to_digit(10));
            let first = iter.next().unwrap();
            let last = iter.last().unwrap_or(first);

            first * 10 + last
        })
        .sum();

    println!("{solution}");
}
