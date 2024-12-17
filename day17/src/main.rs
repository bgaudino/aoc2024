use std::fs;

#[derive(Debug)]
struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    program: Vec<usize>,
    instruction_pointer: usize,
    output: Vec<usize>,
}

impl Computer {
    fn operate(&mut self) {
        let mut jumped = false;
        match self.opcode() {
            0 => self.adv(),
            1 => self.bxl(),
            2 => self.bst(),
            3 => {
                jumped = self.jnz();
            }
            4 => self.bxc(),
            5 => self.out(),
            6 => self.bdv(),
            7 => self.cdv(),
            _ => panic!("Invalid opcode"),
        }
        if !jumped {
            self.next();
        }
    }

    fn run(&mut self) -> String {
        while self.instruction_pointer < self.program.len() - 1 {
            self.operate();
        }
        self.get_output()
    }

    fn get_output(&self) -> String {
        self.output
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn next(&mut self) {
        self.instruction_pointer += 2;
    }

    fn opcode(&self) -> usize {
        self.program[self.instruction_pointer]
    }

    fn literal_operand(&self) -> usize {
        self.program[self.instruction_pointer + 1]
    }

    fn combo_operand(&self) -> usize {
        let operand = self.literal_operand();
        match operand {
            0 | 1 | 2 | 3 => operand,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Invalid operand"),
        }
    }

    fn dv(&self) -> usize {
        self.register_a / 2usize.pow(self.combo_operand() as u32)
    }

    fn adv(&mut self) {
        self.register_a = self.dv();
    }

    fn bxl(&mut self) {
        self.register_b = self.register_b ^ self.literal_operand();
    }

    fn bst(&mut self) {
        self.register_b = self.combo_operand() % 8;
    }

    fn jnz(&mut self) -> bool {
        if self.register_a != 0 {
            self.instruction_pointer = self.literal_operand();
            true
        } else {
            false
        }
    }

    fn bxc(&mut self) {
        self.register_b = self.register_b ^ self.register_c;
    }

    fn out(&mut self) {
        self.output.push(self.combo_operand() % 8);
    }

    fn bdv(&mut self) {
        self.register_b = self.dv();
    }

    fn cdv(&mut self) {
        self.register_c = self.dv();
    }
}

fn get_computer(s: String) -> Computer {
    let mut segments = s.split("\n\n");
    let mut lines = segments.next().unwrap().lines();

    let register_a = parse_register(lines.next().unwrap());
    let register_b = parse_register(lines.next().unwrap());
    let register_c = parse_register(lines.next().unwrap());
    let program = parse_program(segments.next().unwrap());

    Computer {
        register_a,
        register_b,
        register_c,
        program,
        instruction_pointer: 0,
        output: vec![],
    }
}

fn parse_register(s: &str) -> usize {
    s.split_whitespace()
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .parse()
        .unwrap()
}

fn parse_program(s: &str) -> Vec<usize> {
    s.split_whitespace()
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect()
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut computer = get_computer(contents);
    println!("Part 1: {}", computer.run());
}
