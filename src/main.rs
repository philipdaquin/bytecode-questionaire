use std::{sync::mpsc::{channel, Sender, Receiver}, io::{Error, BufReader, BufRead}, fs::File, path::Path};
use glob::glob;

pub enum Operator { 
    Add,
    Multiply,
    Divide,
    Subtract
}
/// ByteCode 
pub enum ByteCode { 
    LoadVal(i32),
    WriteVar { 
        lhs: char, 
        val: Box<ByteCode>
    },
    ReadVar {
        var: Box<ByteCode>
    },
    ReturnValue { 
        node: Box<ByteCode>
    },
    BinaryExpression { 
        op: Operator,
        lhs: Box<ByteCode>,
        rhs: Box<ByteCode>
    },
    Loop { 
        increment_by: Box<ByteCode>,
        from: Box<ByteCode>,
        to: Box<ByteCode>
    }
}

#[derive(Debug)]
pub struct MpscChannels<T> { 
    pub sender: Sender<T>,
    pub receiver: Receiver<T>
}

pub trait Channels<T> { 
    type Item;
    fn send_channel(&self, stack: Vec<Self::Item>) -> Result<(), Error>;
    fn recv_channel(&self) -> Result<Vec<Self::Item>, Error>;
    fn spawn(&self, f: Box<ByteCode>, u: Box<ByteCode>);
}


pub struct ByteCodeInterpreter { 
    constants: Vec<ByteCode>,
    instructions: Vec<ByteCode>

}
/// Small interpreter that takes bytecode as input and interpret the results
impl ByteCodeInterpreter { 
    pub fn interpret_operations(&self, bytecode: &ByteCode) -> i32  { 
        match bytecode { 
            ByteCode::LoadVal(s) => *s,
            ByteCode::WriteVar { lhs, val } => { 
                let lhs_ret = self.interpret_operations(val);
                return lhs_ret
            },
            ByteCode::BinaryExpression { op, lhs, rhs } => { 
                let (lhs_ret, rhs_ret) = (self.interpret_operations(lhs), self.interpret_operations(rhs));
                match op { 
                    Operator::Add => return lhs_ret + rhs_ret,
                    Operator::Multiply => return lhs_ret * rhs_ret,
                    Operator::Subtract => return lhs_ret - rhs_ret,
                    Operator::Divide => return lhs_ret / rhs_ret 
                }
            }
            ByteCode::ReadVar{ var } => { 
                self.interpret_operations(var)
            },
            ByteCode::ReturnValue {node } => { 
                self.interpret_operations(node)
            },

            /// Extend your interpreter with basic loops 
            ByteCode::Loop { increment_by, from, to } => {
                let from_ret = self.interpret_operations(from);
                let to_ret = self.interpret_operations(to);
                let start_by = self.interpret_operations(increment_by) as usize;

                let mut res = 0;
                for i in (from_ret..to_ret).step_by(start_by) { 
                    res = i
                }
                res
            }
        }
        
    }
}
/// Implements Send_Channel, Receiver_Channel and Spawn methods
impl<T> Channels<T> for ByteCodeInterpreter { 
    type Item = T;
    /// Pops the the channel and a value from the stack and send the value on the channel using a blocking send
    fn send_channel(&self, mut stack: Vec<Self::Item>) -> Result<(), Error>{ 
        let (sender, _) : (Sender<T>, _) = channel();
        while let Some(val) = stack.pop() { 
            sender.send(val).unwrap()
        }
        Ok(())
    }
    /// Pops the channel from the stack, receives a value from the channel and pushes the resulting value back on the stack 
    fn recv_channel(&self) -> Result<Vec<T>, Error> {
        let (_, receiver): (_, Receiver<T>) = channel();
        let val_from_sender = receiver.recv().unwrap();
        //  Send value back on the stack 
        let mut stack = Vec::new();
        stack.push(val_from_sender);
        
        Ok(stack)
    }
    /// Pop two functions from the stack and spawn them as concurrent task 
    fn spawn(&self, function_1: Box<ByteCode>, function_2: Box<ByteCode>) {
        let (sender, _) = channel();
        let sender_clone = sender.clone();
        println!("Initialising two concurrent functions!");
        std::thread::spawn(move || { 
            sender.send(function_1).unwrap()
        });
        
        std::thread::spawn(move || { 
            sender_clone.send(function_2).unwrap()
        });
    }
}

#[derive(Debug)]
pub struct FileFinder { 
    pub path: Option<String>,
}
impl FileFinder { 
    /// Write a function that given a directory, recursively finds all files with a given file
    ///extension in that directory and all sub-directories, and counts the number of lines
    ///in the file and prints it to stdout.
    fn find_count_lines(&self, path: Option<String>) { 

        let mut file = match File::open(&path.unwrap()) {
            Err(why) => panic!("couldn't open {}", why),
            Ok(file) => BufReader::new(file),
        };
        
        let mut line_count = 0;
        for _ in file.lines() { 
            line_count = line_count + 1
        }

        println!("Number of lines in the file {:?}", line_count)
    }
}





fn main() {
    println!("Hello, world!");
}

