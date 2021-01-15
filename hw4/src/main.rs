use std::collections::VecDeque;

fn main() {
    println!("Hello");

    /*
    let mut inter = Interpreter::new();
    inter.add_instructions(&["PUSH 2", "PUSH 6", "MUL"]);
    println!("{:?}", inter.run());
    println!("{}", inter.stack[0]);
    */

    let mut inter = Interpreter::new();
    inter.add_instructions(&["PUSH 0", "PUSH 42", "DIV"]);
    inter.forward().unwrap();
    inter.forward().unwrap();
    println!(
        "Instructions: {:?}\nStack: {:?}",
        inter.instructions, inter.stack
    );
    println!("{:?}", inter.forward());

    inter.back().unwrap();
    inter.back().unwrap();
    inter
        .current_instruction()
        .map(|i| *i = String::from("PUSH 2"));

    inter.run().unwrap();
    println!(
        "Instructions: {:?}\nStack: {:?}",
        inter.instructions, inter.stack
    );
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuntimeError {
    DivideByZero,
    StackUnderflow,
    InvalidCommand,
    NoInstructions,
}

#[derive(Debug, Default)]
pub struct Interpreter {
    pub instructions: VecDeque<String>,
    pub stack: Vec<i32>,
    undo_stack: Vec<UndoOP>,
}

#[derive(Debug)]
struct UndoOP {
    instruction: String,
    to_push: Vec<i32>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            instructions: VecDeque::new(),
            stack: Vec::new(),
            undo_stack: Vec::new(),
        }
    }
    pub fn add_instructions(&mut self, instructions: &[&str]) {
        for i in instructions.iter() {
            self.instructions.push_back(i.to_string());
        }
    }
    pub fn current_instruction(&mut self) -> Option<&mut String> {
        self.instructions.front_mut()
    }
    pub fn forward(&mut self) -> Result<(), RuntimeError> {
        let instruction = match self.instructions.front() {
            None => return Err(RuntimeError::NoInstructions),
            Some(instruction) => instruction,
        };

        let mut instruction_iter = instruction.split_whitespace();
        let instruction = instruction_iter.next().unwrap();

        let mut to_push = Vec::new();

        match instruction {
            "PUSH" => match instruction_iter.next() {
                None => return Err(RuntimeError::InvalidCommand),
                Some(x) => match x.parse::<i32>() {
                    Err(_) => return Err(RuntimeError::InvalidCommand),
                    Ok(x) => {
                        to_push.push(x);
                        self.stack.push(x)
                    }
                },
            },
            "POP" => match instruction_iter.next() {
                None => match self.stack.pop() {
                    None => return Err(RuntimeError::StackUnderflow),
                    Some(x) => to_push.push(x),
                },
                Some(_) => return Err(RuntimeError::InvalidCommand),
            },
            "ADD" => match instruction_iter.next() {
                None => match self.stack.pop() {
                    None => return Err(RuntimeError::StackUnderflow),
                    Some(a) => match self.stack.pop() {
                        None => return Err(RuntimeError::StackUnderflow),
                        Some(b) => {
                            to_push.push(a);
                            to_push.push(b);
                            self.stack.push(a + b)
                        }
                    },
                },
                Some(_) => return Err(RuntimeError::InvalidCommand),
            },
            "MUL" => match instruction_iter.next() {
                None => match self.stack.pop() {
                    None => return Err(RuntimeError::StackUnderflow),
                    Some(a) => match self.stack.pop() {
                        None => return Err(RuntimeError::StackUnderflow),
                        Some(b) => {
                            to_push.push(a);
                            to_push.push(b);
                            self.stack.push(a * b)
                        }
                    },
                },
                Some(_) => return Err(RuntimeError::InvalidCommand),
            },
            "SUB" => match instruction_iter.next() {
                None => match self.stack.pop() {
                    None => return Err(RuntimeError::StackUnderflow),
                    Some(a) => match self.stack.pop() {
                        None => return Err(RuntimeError::StackUnderflow),
                        Some(b) => {
                            to_push.push(a);
                            to_push.push(b);
                            self.stack.push(a - b)
                        }
                    },
                },
                Some(_) => return Err(RuntimeError::InvalidCommand),
            },
            "DIV" => match instruction_iter.next() {
                None => match self.stack.pop() {
                    None => return Err(RuntimeError::StackUnderflow),
                    Some(a) => match self.stack.pop() {
                        None => return Err(RuntimeError::StackUnderflow),
                        Some(0) => return Err(RuntimeError::DivideByZero),
                        Some(b) => {
                            to_push.push(a);
                            to_push.push(b);
                            self.stack.push(a / b)
                        }
                    },
                },
                Some(_) => return Err(RuntimeError::InvalidCommand),
            },
            _ => return Err(RuntimeError::InvalidCommand),
        };

        let undo_operation = self.instructions.pop_front();
        self.undo_stack.push(UndoOP {
            instruction: undo_operation.unwrap(),
            to_push,
        });

        Ok(())
    }
    pub fn run(&mut self) -> Result<(), RuntimeError> {
        loop {
            match self.forward() {
                Err(RuntimeError::NoInstructions) => return Ok(()),
                Err(e) => return Err(e),
                _ => (),
            }
        }
    }
    pub fn back(&mut self) -> Result<(), RuntimeError> {
        let mut operation = match self.undo_stack.pop() {
            None => return Err(RuntimeError::NoInstructions),
            Some(x) => x,
        };

        let instruction = operation.instruction.split_whitespace().next().unwrap();

        match instruction {
            "PUSH" => {
                self.stack.pop();
            }
            "POP" => self.stack.push(operation.to_push.pop().unwrap()),
            "ADD" | "MUL" | "SUB" | "DIV" => {
                self.stack.pop();
                self.stack.push(operation.to_push.pop().unwrap());
                self.stack.push(operation.to_push.pop().unwrap());
            }
            _ => unreachable!(),
        }

        self.instructions.push_front(operation.instruction);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Interpreter;
    use crate::RuntimeError;

    #[test]
    fn test_forward() {
        let mut inter = Interpreter::new();
        inter.add_instructions(&["PUSH 0", "PUSH 6", "ADD"]);
        inter.run().unwrap();
        assert_eq!(inter.stack, vec![6]);
        assert_eq!(inter.instructions, Vec::<String>::new());

        let mut inter = Interpreter::new();
        inter.add_instructions(&["PUSH 0", "PUSH 6", "DIV"]);
        assert_eq!(inter.run(), Err(RuntimeError::DivideByZero));

        let mut inter = Interpreter::new();
        inter.add_instructions(&["PUSH 3", "PUSH 6", "POP", "PUSH 17", "ADD"]);
        inter.run().unwrap();
        assert_eq!(inter.stack, vec![20]);
        assert_eq!(inter.instructions, Vec::<String>::new());

        let mut inter = Interpreter::new();
        inter.add_instructions(&["PUSH 0", "MUL 6"]);
        assert_eq!(inter.run(), Err(RuntimeError::InvalidCommand));

        let mut inter = Interpreter::new();
        inter.add_instructions(&["PUSH 0", "MUL"]);
        assert_eq!(inter.run(), Err(RuntimeError::StackUnderflow));

        let mut inter = Interpreter::new();
        inter.add_instructions(&["PUSH", "MUL"]);
        assert_eq!(inter.run(), Err(RuntimeError::InvalidCommand));

        let mut inter = Interpreter::new();
        inter.add_instructions(&["PUSH 0", "POP", "PUSH 13", "SUB"]);
        assert_eq!(inter.run(), Err(RuntimeError::StackUnderflow));
    }

    #[test]
    fn test_back() {
        let mut inter = Interpreter::new();
        inter.add_instructions(&["PUSH 3", "PUSH 42", "DIV"]);
        inter.forward().unwrap();
        inter.forward().unwrap();
        inter.forward().unwrap();
        assert_eq!(inter.stack, vec![14]);
        assert_eq!(inter.instructions, Vec::<String>::new());

        inter.back().unwrap();
        inter.back().unwrap();
        inter
            .current_instruction()
            .map(|i| *i = String::from("PUSH 12"));
        inter.run().unwrap();
        assert_eq!(inter.stack, vec![4]);

        let mut inter = Interpreter::new();
        inter.add_instructions(&["PUSH 3", "PUSH 42"]);
        inter.run().unwrap();
        assert_eq!(inter.stack, vec![3, 42]);
        assert_eq!(inter.instructions, Vec::<String>::new());

        inter.back().unwrap();
        assert_eq!(inter.stack, vec![3]);
        assert_eq!(inter.instructions, vec!["PUSH 42"]);

        let mut inter = Interpreter::new();
        inter.add_instructions(&["PUSH 3", "PUSH 42", "POP"]);
        inter.run().unwrap();
        assert_eq!(inter.stack, vec![3]);
        assert_eq!(inter.instructions, Vec::<String>::new());

        inter.back().unwrap();
        assert_eq!(inter.stack, vec![3, 42]);
        assert_eq!(inter.instructions, vec!["POP"]);

        let mut inter = Interpreter::new();
        inter.add_instructions(&["PUSH 3", "PUSH 42", "ADD"]);
        inter.run().unwrap();
        assert_eq!(inter.stack, vec![45]);
        assert_eq!(inter.instructions, Vec::<String>::new());

        inter.back().unwrap();
        assert_eq!(inter.stack, vec![3, 42]);
        assert_eq!(inter.instructions, vec!["ADD"]);
        inter
            .current_instruction()
            .map(|i| *i = String::from("MUL"));
        assert_eq!(inter.stack, vec![3, 42]);
        inter.run().unwrap();
        assert_eq!(inter.stack, vec![3 * 42]);
        assert_eq!(inter.instructions, Vec::<String>::new());
    }
    #[test]
    fn test_basic() {
        let mut interpreter = Interpreter::new();
        interpreter.add_instructions(&["PUSH 1", "PUSH 2", "PUSH 3", "ADD"]);

        assert_eq!(
            interpreter.instructions,
            &["PUSH 1", "PUSH 2", "PUSH 3", "ADD",]
        );
        assert_eq!(interpreter.stack, &[]);

        interpreter.forward().unwrap();
        interpreter.forward().unwrap();
        interpreter.forward().unwrap();

        assert_eq!(interpreter.instructions, &["ADD"]);
        assert_eq!(interpreter.stack, &[1, 2, 3]);

        interpreter.run().unwrap();

        assert_eq!(interpreter.instructions.len(), 0);
        assert_eq!(interpreter.stack, &[1, 5]);

        interpreter.back().unwrap();
        interpreter.back().unwrap();

        assert_eq!(interpreter.instructions, &["PUSH 3", "ADD",]);
        assert_eq!(interpreter.stack, &[1, 2]);

        interpreter.add_instructions(&["ADD", "ADD"]);

        assert_eq!(interpreter.run(), Err(RuntimeError::StackUnderflow));
        assert_eq!(interpreter.current_instruction().unwrap(), "ADD");
    }
}
