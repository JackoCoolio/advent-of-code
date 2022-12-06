use std::fs::read;

use day_05::{Stacks, Instruction};

fn main() {
    let mut stacks = Stacks::load_from_file("input.txt");

    let data = read("input.txt").unwrap();
    let data = String::from_utf8(data).unwrap();

    let instrs: Vec<Instruction> = data.lines().filter_map(|line| Instruction::try_from_string(line).ok()).collect();

    for instr in instrs.into_iter() {
        stacks.execute_instruction(instr, true);
    }

    println!("{}", stacks.get_tops().into_iter().collect::<String>())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn it_works() {
        let mut stacks = Stacks::load_from_string(INPUT);

        for instr in INPUT.lines().filter_map(|line| Instruction::try_from_string(line).ok()) {
            stacks.execute_instruction(instr, true);
        }

        assert_eq!(stacks.get_tops().into_iter().copied().collect::<Vec<char>>(), vec!['M', 'C', 'D']);
    }
}
