use std::io::prelude::*;
use std::fs::File;


#[derive(Debug, PartialEq, Clone, Copy)]
enum Register {
    A, B, C, D
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Cpy, Inc, Dec, Jnz
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Arg {
    Reg(Register),
    Literal(i32)
}

#[derive(Debug)]
struct Instruction {
    lineno: usize,
    op: Op,
    lhs: Arg,
    rhs: Option<Arg>
}

struct Program {
    instructions: Vec<Instruction>
}

#[derive(Debug)]
struct State {
    a: i32,
    b: i32,
    c: i32,
    d: i32
}


impl State {
    fn new(a: i32, b: i32, c: i32, d: i32) -> State {
        State {
            a: a,
            b: b,
            c: c,
            d: d
        }
    }
    
    fn read(&self, reg: Register) -> i32 {
        match reg {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d
        }
    }

    fn write(&mut self, reg: Register, val: i32) {
        match reg {
            Register::A => self.a = val,
            Register::B => self.b = val,
            Register::C => self.c = val,
            Register::D => self.d = val
        }
    }
}


fn parse_arg(arg_str: &str) -> Arg {
    match arg_str {
        "a" => Arg::Reg(Register::A),
        "b" => Arg::Reg(Register::B),
        "c" => Arg::Reg(Register::C),
        "d" => Arg::Reg(Register::D),
         x  => Arg::Literal(x.parse::<i32>().unwrap()),
    }
}


fn parse_instr(lineno: usize, line: &str) -> Instruction {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    Instruction {
        lineno: lineno,
        op: match tokens.get(0) {
            Some(x) => match *x {
                "cpy" => Op::Cpy,
                "inc" => Op::Inc,
                "dec" => Op::Dec,
                "jnz" => Op::Jnz,
                _ => panic!()
            },
            None => panic!()
        },
        lhs: match tokens.get(1) {
            Some(arg) => parse_arg(arg),
            None => panic!()
        },
        rhs: match tokens.get(2) {
            Some(arg) => Some(parse_arg(arg)),
            None => None
        }
    }
}


fn parse_prog(source: String) -> Program {
    let instructions: Vec<Instruction> = source
        .lines()
        .enumerate()
        .map(|(n, l)| parse_instr(n, l))
        .collect();
    Program {
        instructions: instructions,
    }
}


fn execute_program<'a>(prog: &Program, state: &'a mut State) -> &'a mut State {
    let mut line = 0;
    while let Some(instr) = prog.instructions.get(line) {
        match instr.op {
            Op::Cpy => {
                let dst = match instr.rhs {
                    Some(Arg::Reg(r)) => r,
                    _ => panic!()
                };
                match instr.lhs {
                    Arg::Reg(src) => {
                        let val = state.read(src);
                        state.write(dst, val);
                    },
                    Arg::Literal(x) => state.write(dst, x)
                };
            },
            op @ Op::Inc |
            op @ Op::Dec => {
                let src = match instr.lhs {
                    Arg::Reg(r) => r,
                    _ => panic!()
                };
                let val = state.read(src);
                state.write(src, val + if op == Op::Inc { 1 } else { -1 });
            },
            Op::Jnz => {
                let cond = match instr.lhs {
                    Arg::Literal(x) => x,
                    Arg::Reg(r) => state.read(r)
                };
                if cond != 0 {
                    match instr.rhs {
                        Some(Arg::Literal(x)) => {
                            line = (line as i32 + x) as usize;
                            continue;
                        },
                        _ => panic!()
                    };
                };
            }
        }
        line += 1;
    }
    state
}


fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let prog: Program = parse_prog(s);

    // Part 1
    let mut state = State::new(0, 0, 0, 0);
    println!("final state (part 1): {:?}", execute_program(&prog, &mut state));

    // Part 2
    state = State::new(0, 0, 1, 0);
    println!("final state (part 2): {:?}", execute_program(&prog, &mut state));
}
