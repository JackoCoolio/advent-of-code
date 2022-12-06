use std::fs::read;

#[derive(Default, Debug)]
pub struct Stacks(Vec<Stack>, usize);

#[derive(Default, Debug)]
pub struct Stack(Vec<char>);

#[derive(Debug)]
pub struct Instruction {
    n: usize,
    from: usize,
    to: usize,
}

impl Stacks {
    pub fn load_from_string(s: &str) -> Stacks {
        let mut stacks = Vec::<Stack>::new();

        let num_stacks = (s.lines().next().unwrap().len() + 1) / 4;
        for _ in 0..num_stacks {
            stacks.push(Default::default());
        }

        'lines: for line in s.lines() {
            for (i, c) in line.chars().enumerate() {
                if i / 4 >= num_stacks {
                    break;
                }

                if i % 4 == 1 {
                    if c.is_ascii_digit() {
                        break 'lines;
                    }

                    if c.is_alphabetic() {
                        stacks[i / 4].0.push(c);
                    }
                }
            }
        }

        for stack in &mut stacks {
            stack.0.reverse();
        }

        Stacks(stacks, num_stacks)
    }

    pub fn load_from_file(path: &str) -> Stacks {
        let bytes = read(path).unwrap();
        Self::load_from_string(&String::from_utf8(bytes).unwrap())
    }

    /// Moves a crate from one stack to the other.
    pub fn mov(&mut self, from: usize, to: usize) {
        let c = self.0[from].0.pop().unwrap();
        self.0[to].0.push(c);
    }

    /// Moves `n` elements from one stack to the other.
    pub fn move_n(&mut self, from: usize, to: usize, n: usize) {
        for _ in 0..n {
            self.mov(from, to);
        }
    }

    /// Moves `n` elements from one stack to the other, preserving order.
    pub fn move_n_at_once(&mut self, from: usize, to: usize, n: usize) {
        let height = self.0[from].0.len();
        let crates = self.0[from].0.split_off(height - n);
        self.0[to].0.extend(crates);
    }

    /// Executes an instruction with the format of "move <n> from <from> to <to>"
    pub fn execute_instruction(&mut self, Instruction {from, to, n}: Instruction, at_once: bool) {
        if at_once {
            self.move_n_at_once(from, to, n);
        } else {
            self.move_n(from, to, n);
        }
    }

    /// Gets the crates at the top of the stacks.
    pub fn get_tops(&self) -> Vec<&char> {
        self.0.iter().map(|s| s.0.last().unwrap_or(&' ')).collect()
    }
}

impl Instruction {
    pub fn try_from_string(instr: &str) -> Result<Self, String> {
        match &instr.split_whitespace().collect::<Vec<&str>>()[..] {
            &["move", n, "from", from, "to", to] => match (
                n.parse::<usize>(),
                from.parse::<usize>(),
                to.parse::<usize>(),
            ) {
                (Ok(n), Ok(from), Ok(to)) => Ok(Self { n, from: from - 1, to: to - 1 }),
                _ => Err("invalid instruction parameters".into()),
            },
            _ => Err("invalid instruction".into()),
        }
    }
}
