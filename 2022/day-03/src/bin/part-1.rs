use day_03::{get_priority, load_file, load_rucksacks, Rucksack};

fn main() {
    let data = load_file("input.txt");
    let rucksacks = load_rucksacks(&data);

    println!("{}", get_sum_of_intersections(&rucksacks));
}

fn get_sum_of_intersections(rucksacks: &[Rucksack]) -> u32 {
    rucksacks
        .iter()
        .map(|rucksack| get_priority(*rucksack.get_intersection()) as u32)
        .sum()
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
        assert_eq!(get_sum_of_intersections(&rucksacks), 157);
    }
}
