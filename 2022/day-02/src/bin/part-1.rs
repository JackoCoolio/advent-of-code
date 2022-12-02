use day_02::{load_mm_rounds, Move, load_file};

fn main() {
    let s = load_file("input.txt");
    let rounds = load_mm_rounds(&s);

    let score = get_score(rounds);

    println!("{}", score);
}

fn get_score(rounds: Vec<(Move, Move)>) -> u32 {
    let mut score = 0_u32;
    for (you, me) in rounds.into_iter() {
        score += if me > you {
            6
        } else if me == you {
            3
        } else {
            0
        } + me.into_score();
    }
    score
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"A Y
B X
C Z
"#;

    #[test]
    fn it_works() {
        let rounds = load_mm_rounds(INPUT);
        assert_eq!(get_score(rounds), 15);
    }
}

