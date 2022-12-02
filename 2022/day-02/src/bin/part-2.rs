use day_02::{RoundResult, Move, load_file, load_mr_rounds};

fn main() {
    let s = load_file("input.txt");

    let rounds = load_mr_rounds(&s);

    let score = get_score(rounds);

    println!("{}", score);
}

fn get_score(rounds: Vec<(Move, RoundResult)>) -> u32 {
    rounds.into_iter().map(|(mov, res)| {
        match res {
            RoundResult::Win => 6 + mov.get_sup().into_score(),
            RoundResult::Draw => 3 + mov.into_score(),
            RoundResult::Loss => mov.get_inf().into_score(),
        }
    }).sum()
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
        let rounds = load_mr_rounds(INPUT);
        assert_eq!(get_score(rounds), 12);
    }
}

