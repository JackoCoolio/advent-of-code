fn load_file() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn parse_word_digit(s: &str) -> Option<u32> {
    const WORDS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for (i, word) in WORDS.into_iter().enumerate() {
        if s.starts_with(word) {
            return Some((i + 1).try_into().unwrap());
        }
    }

    None
}

fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut first: Option<u32> = None;
            let mut last: Option<u32> = None;
            for (i, c) in line.char_indices() {
                if let Some(digit) = c.to_digit(10) {
                    if first.is_none() {
                        first = Some(digit);
                    } else {
                        last = Some(digit);
                    }
                }

                // match on words
                if let Some(digit) = parse_word_digit(&line[i..]) {
                    if first.is_none() {
                        first = Some(digit);
                    } else {
                        last = Some(digit);
                    }
                }
            }

            first.unwrap() * 10 + last.or(first).unwrap()
        })
        .sum()
}

fn main() {
    let input = load_file();

    let solution = process(&input);

    println!("{solution}");
}

#[test]
fn test() {
    const INPUT: &str = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;

    assert_eq!(process(INPUT.trim()), 281);
}
