use day_01::{load_file, get_calorie_counts};

fn main() {
    let s = load_file("input.txt").unwrap();
    println!("{}", get_highest_calorie_count(s));
}

fn get_highest_calorie_count(s: String) -> u32 {
    let counts = get_calorie_counts(&s);

    counts[0]
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
    fn it_works() {
        assert_eq!(get_highest_calorie_count(INPUT.to_string()), 24000);
    }
}
