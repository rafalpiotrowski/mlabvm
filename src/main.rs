use std::{io::Read, collections::BTreeMap, panic};


#[derive(Debug)]
enum Instruction {
    LoadVal(i32),
    WriteVar(String),
    ReadVar(String),
    Add,
    Multiply,
    ReturnValue,
    PrintStack,
    Loop(u32)
}

fn parse_instruction(s: &[&str]) -> Instruction {
    use Instruction::*;

    match s {
        ["LOAD_VAL", val] => LoadVal(val.parse::<i32>().unwrap()),
        ["WRITE_VAR", var] => WriteVar(var.to_string()),
        ["READ_VAR", var] => ReadVar(var.to_string()),
        ["ADD"] => Add,
        ["MULTIPLY"] => Multiply,
        ["RETURN_VALUE"] => ReturnValue,
        ["PRINT_STACK"] => PrintStack,
        ["LOOP", count] => Loop(count.parse::<u32>().unwrap()),
        i => panic!("Invalid instruction {:?}", i)
    }
}

type Program<'a> = &'a [Instruction];

type Value = i32;
type Pointer = usize;
type Variables<'a> = BTreeMap<&'a str, Value>;

#[derive(Debug)]
struct Stack(Vec<Value>);

impl Stack {
    fn push(&mut self, v: Value) {
        self.0.push(v);
    }

    fn pop(&mut self) -> Value {
        self.0.pop().expect("trying to pop from empty stack")
    }
}

fn interpret(program: Program) {
    use Instruction::*;

    let mut stack: Stack = Stack(Vec::new());
    let mut pointer: Pointer = 0;
    let mut variables = Variables::new();

    while let Some(instruction) = program.get(pointer) {
        pointer += 1; // move forward in the instruction vector

        match instruction {
            LoadVal(v) => {
                stack.push(*v)
            },
            WriteVar(v) => {
                variables.insert(v.as_str(), stack.pop());
            },
            ReadVar(variable) => {
                let p = get_variable_value(variable.as_str(), &variables);
                match p {
                    Some(value) => stack.push(value),
                    None => panic!("missing variable {variable}")
                }
            },
            Add => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(a + b)
            },
            Multiply => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(a * b)
            },
            Loop(count) => {
                
            },
            ReturnValue => {
                println!("{}", stack.pop())
            },            
            PrintStack => {
                println!("Stack: {:?}", &stack);
            }
        }
    }
}

fn get_variable_value(name: &str, variables: &BTreeMap<&str, Value>) -> Option<Value> {
    let p = variables.get_key_value(name);
    match p {
        Some((_,v)) => Some(*v),
        None => None
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut f = std::fs::File::open(&args[1])?;

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    let lines = buffer
        .split('\n')
        .map(|s| s.split_whitespace().collect::<Vec<_>>())
        .filter(|s| !matches!(s.as_slice(), [] | ["--", ..]))
        .collect::<Vec<_>>();

    let instructions: Vec<Instruction> = lines
        .iter()
        .map(|s| parse_instruction(s.as_slice()))
        .collect();

    println!("Program: {:?}", &instructions);

    interpret(&instructions[..]);

    Ok(())
}

