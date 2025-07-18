use std::collections::HashSet;

static INPUT: &str = include_str!("../input");

fn main() {
    println!("part one: {}", part_one(INPUT));
    println!("part two: {}", part_two(INPUT));
}

#[derive(Debug, Clone)]
struct Instruction {
    op: String,
    arg: i32,
}

#[derive(Debug, Clone)]
struct State {
    program: Vec<Instruction>,
    instruction_pointer: usize,
    accumulator: i32,
    instruction_history: HashSet<usize>,
}

impl State {
    fn new(program: &str) -> Self {
        let program = program
            .lines()
            .map(|line| Instruction {
                op: String::from(&line[..3]),
                arg: line[4..].parse().unwrap(),
            })
            .collect();
        Self {
            program,
            instruction_pointer: 0,
            accumulator: 0,
            instruction_history: HashSet::new(),
        }
    }
}

fn part_one(program: &str) -> i32 {
    let mut state = State::new(program);
    loop {
        let i = state.instruction_pointer;
        if state.instruction_history.contains(&i) {
            return state.accumulator;
        }
        state.instruction_history.insert(i);
        let instruction = &state.program[i];
        match instruction.op.as_str() {
            "nop" => {}
            "acc" => state.accumulator += instruction.arg,
            "jmp" => {
                state.instruction_pointer = (i as i32 + instruction.arg) as usize;
                continue;
            }
            _ => panic!("invalid op"),
        }
        state.instruction_pointer += 1;
    }
}

fn part_two(program: &str) -> i32 {
    let mut stack = vec![State::new(program)];
    while let Some(mut s) = stack.pop() {
        loop {
            let i = s.instruction_pointer;
            if i == s.program.len() {
                return s.accumulator;
            }
            if s.instruction_history.contains(&i) {
                break;
            }
            s.instruction_history.insert(i);
            let instruction = &s.program[i];
            match instruction.op.as_str() {
                "nop" => {
                    let mut new_state = s.clone();
                    new_state.instruction_pointer = (i as i32 + instruction.arg) as usize;
                    stack.push(new_state);
                }
                "acc" => s.accumulator += instruction.arg,
                "jmp" => {
                    let mut new_state = s.clone();
                    new_state.instruction_pointer += 1;
                    stack.push(new_state);
                    s.instruction_pointer = (i as i32 + instruction.arg) as usize;
                    continue;
                }
                _ => panic!("invalid op"),
            }
            s.instruction_pointer += 1;
        }
    }
    panic!("no solution found");
}
