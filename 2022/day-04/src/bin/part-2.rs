use day_04::{load_file, load_pairs, Pair};

fn main() {
    let data = load_file("input.txt");
    let pairs = load_pairs(&data);

    println!("{}", count_overlapping_pairs(&pairs));
}

fn count_overlapping_pairs(pairs: &[Pair]) -> usize {
    let iter = pairs.iter();
    iter.filter(|p| p.overlaps()).count()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn it_works() {
        let pairs = load_pairs(INPUT);
        assert_eq!(count_overlapping_pairs(&pairs), 4);
    }
}

