use day_01::{get_calorie_counts, load_file};

fn main() {
    let s = load_file("input.txt").unwrap();
    println!("{:?}", get_three_highest_calorie_counts(s).iter().sum::<u32>());
}

fn get_three_highest_calorie_counts(s: String) -> Vec<u32> {
    let counts = get_calorie_counts(&s);

    Vec::from(&counts[0..3])
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

"#;

    #[test]
    fn it_works( ){
        assert_eq!(get_three_highest_calorie_counts(INPUT.to_string()), vec![24000, 11000, 10000]);
    }
}
