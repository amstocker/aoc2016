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


fn write_register(state: &mut State, reg: Register, val: i32) {
    match reg {
        Register::A => state.a = val,
        Register::B => state.b = val,
        Register::C => state.c = val,
        Register::D => state.d = val
    }
}


fn read_register(state: &State, reg: Register) -> i32 {
    match reg {
        Register::A => state.a,
        Register::B => state.b,
        Register::C => state.c,
        Register::D => state.d
    }
}


fn execute_program<'a>(prog: &Program, state: &'a mut State) -> &'a mut State {
    let mut line = 0;
    while let Some(instr) = prog.instructions.get(line) {
        println!("{:?}", instr);
        match instr.op {
            Op::Cpy => {
                let dst = match instr.rhs {
                    Some(Arg::Reg(r)) => r,
                    _ => panic!()
                };
                match instr.lhs {
                    Arg::Reg(src) => {
                        let val = read_register(state, src);
                        write_register(state, dst, val);
                    },
                    Arg::Literal(x) => write_register(state, dst, x)
                };
            },
            op @ Op::Inc |
            op @ Op::Dec => {
                let src = match instr.lhs {
                    Arg::Reg(r) => r,
                    _ => panic!()
                };
                let val = read_register(state, src);
                write_register(state, src, val + if op == Op::Inc { 1 } else { -1 });
            },
            Op::Jnz => {
                let cond = match instr.lhs {
                    Arg::Literal(x) => x,
                    Arg::Reg(r) => read_register(state, r)
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
        println!("{:?}", state);
        line += 1;
    }
    state
}


fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let prog: Program = parse_prog(s);
    for i in &prog.instructions {
        println!("{:?}", i);
    }

    let mut state = State {
        a: 0,
        b: 0,
        c: 0,
        d: 0
    };
    println!("final state: {:?}", execute_program(&prog, &mut state));
}