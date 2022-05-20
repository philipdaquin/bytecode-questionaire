use std::{sync::mpsc::{channel, Sender, Receiver}, io::Error};

/*
function f() {
    x = 1                       LOAD_VAL 1
                                WRITE_VAR ‘x’
    y = 2                       LOAD_VAL 2
                                WRITE_VAR ‘y’
    return (x + 1) * y          READ_VAR ‘x’
                                LOAD_VAL 1
                                ADD
                                READ_VAR ‘y’
                                MULTIPLY
                                RETURN_VALUE
}
*/
/// Add a data type Bytecode that can represent bytecode 
pub enum ByteCode<T> { 
    LoadVal(i32),
    WriteVar(char),
    ReadVar(char),
    Add, 
    Subtract,
    Multiply,
    ReturnValue,
    Loop(Vec<Loops<T>>),
    SendChannel,
    RecvChannel, 
    Spawn
}

/// Extend your interpreter with loops 
#[derive(Clone, Debug, PartialEq)]
pub struct Loops<T> { 
    pub start: T,
    pub end: T
}

pub trait CustomLoop {
    type Item; 
    fn next(&self) -> Self::Item;
}
impl<T> CustomLoop for Loops<T> where T: Clone { 
    type Item = T;
    fn next(&self) -> Self::Item {
        self.end.clone()
    }
} 

#[derive(Debug)]
pub struct MpscChannels<T> { 
    pub sender: Sender<T>,
    pub receiver: Receiver<T>
}

pub trait Channels<T> { 
    type Item;
    fn send_channel(&self,stack: Vec<Self::Item>) -> Result<(), Error>;
    fn recv_channel(&self) -> Result<Vec<Self::Item>, Error>;
    fn spawn(&self, f: ByteCode<T>, u: ByteCode<T>);
}



impl<T> Channels<T> for MpscChannels<T> { 
    type Item = T;
    /// Pops the channel and a value from the stack and send the value on the channel 
    /// using a blocking send 
    fn send_channel(&self, mut stack: Vec<Self::Item>) -> Result<(), Error>{ 
        let (sender, _) : (Sender<T>, _) = channel();
        while let Some(val) = stack.pop() { 
            sender.send(val).unwrap()
        }
        Ok(())
    }
    ///  Pops the channel from the stack, receives a value from the channel and push the resulting 
    /// value back onto the stack 
    fn recv_channel(&self) -> Result<Vec<T>, Error> {
        let (_, receiver): (_, Receiver<T>) = channel();
        let val_from_sender = receiver.recv().unwrap();
        //  Send value back on the stack 
        let mut stack = Vec::new();
        stack.push(val_from_sender);
        
        Ok(stack)
    }
    
    /// Pop two functions from the stack and spawn them as concurrent task 
    fn spawn(&self, function_1: ByteCode<T>, function_2: ByteCode<T>) {
        
        let MpscChannels {sender, receiver} = self;

        std::thread::spawn(move || { 
            sender.send(function_1).unwrap()
        });
        std::thread::spawn(move || { 
            sender.send().unwrap()
        });


    }


}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
pub mod test { 
    use super::*;
    #[test]
    fn test_iter() { 
        let num: Loops<_> = Loops { start: 1, end: 2 };
        let loops = num.next();

        assert_eq!(2, loops);
        println!("{:?}", loops)
    }

}