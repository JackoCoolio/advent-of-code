use day_03::{get_priority, load_file, load_rucksacks, Rucksack};

fn main() {
    let data = load_file("input.txt");
    let rucksacks = load_rucksacks(&data);
    println!("{}", get_sum_of_badges(&rucksacks));
}

fn get_sum_of_badges(rucksacks: &[Rucksack]) -> u32 {
    rucksacks
        .chunks(3)
        .map(get_badge)
        .cloned()
        .map(|c| get_priority(c) as u32)
        .sum()
}

fn get_badge(rucksacks: &[Rucksack]) -> &char {
    let (base, others) = rucksacks.split_at(1);
    others
        .iter()
        .map(|rs| rs.get_union())
        .fold(base[0].get_union(), |acc, b| {
            acc.intersection(&b).cloned().collect()
        })
        .into_iter()
        .next()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn it_works() {
        let rucksacks = load_rucksacks(INPUT);
        assert_eq!(get_sum_of_badges(&rucksacks), 70);
    }
}
