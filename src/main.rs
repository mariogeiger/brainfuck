use std::fs;
use std::io;
use std::iter::Iterator;

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
    Inc,
    Dec,
    Print,
    Read,
    Loop(Vec<Instruction>),
}

fn parse(input: &mut dyn Iterator<Item = char>) -> Vec<Instruction> {
    let mut instruction = Vec::new();

    loop {
        let c = input.next();
        match c {
            Some('[') => {
                instruction.push(Instruction::Loop(parse(input)));
            }
            Some(']') => {
                return instruction;
            }
            Some('<') => {
                instruction.push(Instruction::Left);
            }
            Some('>') => {
                instruction.push(Instruction::Right);
            }
            Some('+') => {
                instruction.push(Instruction::Inc);
            }
            Some('-') => {
                instruction.push(Instruction::Dec);
            }
            Some('.') => {
                instruction.push(Instruction::Print);
            }
            Some(',') => {
                instruction.push(Instruction::Read);
            }
            None => {
                return instruction;
            }
            _ => {}
        }
    }
}

struct StdIn {
    input: String,
    pos: usize,
}

impl StdIn {
    fn new() -> StdIn {
        StdIn {
            input: String::new(),
            pos: 0,
        }
    }

    fn read(&mut self) -> char {
        while self.pos >= self.input.len() {
            self.input.clear();
            io::stdin().read_line(&mut self.input).unwrap();
            self.pos = 0;
        }

        let c = self.input.chars().nth(self.pos);
        self.pos += 1;
        c.unwrap()
    }
}

struct Tape {
    cells: Vec<u8>,
    pos: usize,
}

impl Tape {
    fn new() -> Tape {
        Tape {
            cells: vec![0],
            pos: 0,
        }
    }

    fn left(&mut self) {
        if self.pos == 0 {
            self.cells.insert(0, 0);
        } else {
            self.pos -= 1;
        }
    }

    fn right(&mut self) {
        self.pos += 1;
        if self.pos == self.cells.len() {
            self.cells.push(0);
        }
    }

    fn inc(&mut self) {
        self.cells[self.pos] += 1;
    }

    fn dec(&mut self) {
        self.cells[self.pos] -= 1;
    }

    fn print(&self) {
        print!("{}", self.cells[self.pos] as char);
    }

    fn read(&mut self, stdin: &mut StdIn) {
        let c = stdin.read();
        self.cells[self.pos] = c as u8;
    }
}

fn execute(i: &Instruction, tape: &mut Tape, stdin: &mut StdIn) {
    match i {
        Instruction::Left => {
            tape.left();
        }
        Instruction::Right => {
            tape.right();
        }
        Instruction::Inc => {
            tape.inc();
        }
        Instruction::Dec => {
            tape.dec();
        }
        Instruction::Print => {
            tape.print();
        }
        Instruction::Read => {
            tape.read(stdin);
        }
        Instruction::Loop(instructions) => {
            while tape.cells[tape.pos] != 0 {
                for i in instructions {
                    execute(i, tape, stdin);
                }
            }
        }
    }
}

fn main() {
    // get filename from command line
    let mut args = std::env::args();
    let _ = args.next();
    let filename = args.next().unwrap();

    // read file
    let contents: String = fs::read_to_string(filename).unwrap().parse().unwrap();

    // parse input
    let instructions = parse(&mut contents.chars());

    let mut tape = Tape::new();
    let mut stdin = StdIn::new();

    for i in &instructions {
        execute(i, &mut tape, &mut stdin);
    }
}
