#[derive(Debug)]
struct VM {
    stack: Vec<i32>,
}

impl VM {
    fn new() -> VM {
        VM {
            stack: Vec::new(),
        }
    }

    fn push(&mut self, value: i32) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Option<i32> {
        self.stack.pop()
    }

    fn add(&mut self) {
        if self.stack.len() < 2 {
            println!("Error: Not enough values on the stack to perform addition.");
            return;
        }
        let a = self.pop().unwrap();
        let b = self.pop().unwrap();
        self.push(a + b);
    }

    fn sub(&mut self) {
        if self.stack.len() < 2 {
            println!("Error: Not enough values on the stack to perform subtraction.");
            return;
        }
        let a = self.pop().unwrap();
        let b = self.pop().unwrap();
        self.push(a - b);
    }

    fn mul(&mut self) {
        if self.stack.len() < 2 {
            println!("Error: Not enough values on the stack to perform multiplication.");
            return;
        }
        let a = self.pop().unwrap();
        let b = self.pop().unwrap();
        self.push(a * b);
    }

    fn div(&mut self) {
        if self.stack.len() < 2 {
            println!("Error: Not enough values on the stack to perform division.");
            return;
        }
        let a = self.pop().unwrap();
        let b = self.pop().unwrap();
        if b == 0 {
            println!("Error: Division by zero.");
            self.push(b);
            self.push(a);
            return;
        }
        self.push(a / b);
    }

    fn run(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            match instruction {
                Instruction::Push(value) => self.push(value),
                Instruction::Pop => {
                    self.pop();
                }
                Instruction::Add => self.add(),
                Instruction::Sub => self.sub(),
                Instruction::Mul => self.mul(),
                Instruction::Div => self.div(),
            }
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Push(i32),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
}

fn main() {
    let mut vm = VM::new();
    
    let instructions = vec![
        Instruction::Push(2),
        Instruction::Push(3),
        Instruction::Pop,
        Instruction::Push(4),
        Instruction::Add,
        Instruction::Push(10),
        Instruction::Sub,
        Instruction::Push(6),
        Instruction::Mul,
        Instruction::Div,
    ];

    vm.run(instructions);

    println!("Final stack state: {:?}", vm.stack);
}
