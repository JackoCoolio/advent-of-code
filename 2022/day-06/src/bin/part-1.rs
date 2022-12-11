use std::collections::{hash_map::RandomState, HashSet};

use day_06::{find_first_n_unique, load_file};

fn main() {
    let input = load_file("input.txt");
    let s = input.chars().collect::<Vec<_>>();

    let first_signal = find_start_of_packet(&s);

    println!("{}", first_signal);
}

fn find_start_of_packet(s: &[char]) -> usize {
    find_first_n_unique(s, 4)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_find_first_signal() {
        assert_eq!(find_start_of_packet(&INPUT.chars().collect::<Vec<_>>()), 7);
    }
}
