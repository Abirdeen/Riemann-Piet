use std::io::BufRead;

use crate::canvas::{Coordinate, Codel, CodelBlock, Canvas};

pub enum Command<'a> {
    Push(&'a dyn Fn(&mut Stack, i64) -> ()),
    StackOps(&'a dyn Fn(&mut Stack) -> ()),
    Interpreter(&'a dyn Fn(&mut Interpreter) -> ()),
    InputNum(&'a dyn Fn(&mut Stack, i64) -> ()),
    InputChar(&'a dyn Fn(&mut Stack, char) -> ()),
    OutputNum(&'a dyn Fn(&mut Stack) -> i64),
    OutputChar(&'a dyn Fn(&mut Stack)-> Option<char>)
}

type Stack = Vec<i64>;
type DirectionalPointer = &'static str;
type CodelChooser = &'static str;

mod dp {
    pub const NORTH: &'static str = "North";
    pub const EAST: &'static str = "East";
    pub const WEST: &'static str = "West";
    pub const SOUTH: &'static str = "South";
}

mod cc {
    pub const LEFT: &'static str = "Left";
    pub const RIGHT: &'static str = "Right";
}

pub struct Interpreter {
    stack: Stack,
    dp: DirectionalPointer,
    cc: CodelChooser,
    current_coordinate: Coordinate
}
mod commands {
    use std::collections::VecDeque;

    use crate::interpreter::{Interpreter, Stack};

    // Pushes the number of codels in the previous color block onto the stack.
    pub fn push(stack: &mut Stack, value: i64) {
        stack.push(value)
    }
    // Pops the top value off the stack.
    pub fn pop(stack: &mut Stack) -> () {
        stack.pop();
    }
    // Pops the top two values off the stack, adds them up, and pushes the sum back onto the stack.
    pub fn add(stack: &mut Stack) -> () {
        let x = stack.pop().unwrap_or(0);
        let y = stack.pop().unwrap_or(0);
        stack.push(x+y);
    }
    // Pops the top two values off the stack, subtracts the top value from the second-top value, and pushes the difference back onto the stack. Note that if the top value is X and the next value Y, this means that Y - X will be pushed, not X - Y.
    pub fn subtract(stack: &mut Stack) -> () {
        let x = stack.pop().unwrap_or(0);
        let y = stack.pop().unwrap_or(0);
        stack.push(y-x);
    }
    // Pops the top two values off the stack, multiplies them together, and pushes the product back onto the stack.
    pub fn multiply(stack: &mut Stack) -> () {
        let x = stack.pop().unwrap_or(0);
        let y = stack.pop().unwrap_or(0);
        stack.push(x*y);
    }
    // Pops the top two values off the stack, performs integer division (Python equivalent of //) on the second-top value divided by the top value, and pushes the quotient back onto the stack. This has the same X/Y property as subtraction.
    pub fn divide(stack: &mut Stack) -> () {
        let x = stack.pop().unwrap_or(1);
        let y = stack.pop().unwrap_or(0);
        stack.push(y/x);
    }
    // Pops the top two values off the stack, divided the second-top value by the top value, and pushes the remainder back onto the stack. This has the same X/Y property as subtraction.
    pub fn modulo(stack: &mut Stack) -> () {
        let x = stack.pop().unwrap_or(1);
        let y = stack.pop().unwrap_or(0);
        stack.push(y.rem_euclid(x));
    }
    // Pops the top value off the stack. If the value is 0, it pushes 1 onto the stack. Otherwise, it pushes 0.
    pub fn not(stack: &mut Stack) -> () {
        let x = stack.pop().unwrap_or(0);
        stack.push(if x==0 {1} else {0});
    }
    // Pops the top two values off the stack. If the second-top value is greater than the top value, it pushes 1 onto the stack. Otherwise, it pushes 0. This has the same X/Y property as subtraction.
    pub fn greater(stack: &mut Stack) -> () {
        let x = stack.pop().unwrap_or(1);
        let y = stack.pop().unwrap_or(0);
        stack.push(if x<y {1} else {0});
    }
    // Pushes a copy of the top value onto the stack.
    pub fn duplicate(stack: &mut Stack) -> () {
        let x = stack.pop().unwrap_or(0);
        stack.push(x);
        stack.push(x);
    }
    // Pops the top two values off the stack, and then rotates the top Y values on the stack up by X, wrapping values that pass the top around to the bottom of the rolled portion, where X is the first value popped (top of the stack), and Y is the second value popped (second on the stack). (Example: If the stack is currently 1,2,3, with 3 at the top, and then you push 3 and then 1, and then roll, the new stack is 3,1,2.)
    pub fn roll(stack: &mut Stack) -> () {
        let x = stack.pop().unwrap_or(0);
        let y = stack.pop().unwrap_or(0).min(stack.len() as i64);

        if y==0 {return ()}

        let mut vals: VecDeque<i64> = VecDeque::new();
        for _i in 0..y {
            // Justify: by construction of y, we should never pop more than the full stack
            vals.push_back(stack.pop().unwrap());
        }

        if x<0 {
            vals.rotate_left(x.abs().rem_euclid(vals.len() as i64) as usize);
        } else if x>0 {
            vals.rotate_right(x.rem_euclid(vals.len() as i64) as usize);
        }

        for _i in 0..vals.len() {
            stack.push(vals.pop_back().unwrap_or(0));
        }
    }
    // Pops the top value off the stack, then rotates the DP one step clockwise that many times (anti-clockwise if the value is negative).
    pub fn pointer(interpreter: &mut Interpreter) -> () {
        let stack = interpreter.stack();
        let n = stack.pop().unwrap_or(0);
        interpreter.rotate_dp_right(n);
    }
    // Pops the top value off the stack, then switches the state of the CC that many times (absolute value if the value is negative).
    pub fn switch(interpreter: &mut Interpreter) -> () {
        let stack = interpreter.stack();
        let n = stack.pop().unwrap_or(0);
        interpreter.flip_n_cc(n);
    }
    // Takes an input, either as a character or a number. If the input is a number, that value is pushed onto the stack. If it's a character, its Unicode value is pushed onto the stack.
    pub fn input_num(stack: &mut Stack, input: i64) -> () {
        stack.push(input);
    }
    pub fn input_char(stack: &mut Stack, input: char) -> () {
        stack.push(input as i64);
        }
    // Pops the top value off the stack. If a number should be printed, the value itself will be printed. If a character should be printed, then its corresponding Unicode character will be printed.
    pub fn output_num(stack: &mut Stack) -> i64 {
        stack.pop().unwrap_or(0)
    }
    pub fn output_char(stack: &mut Stack) -> Option<char> {
        let char_int = stack.pop().unwrap_or(0);
        if char_int < 0 || char_int > 0x10FFFF {
            return None
        }
        let char_unic = char::from_u32(char_int as u32);
        return char_unic
        }

}

fn get_command<'a>(canvas: &mut Canvas, from_coord: Coordinate, to_coord: Coordinate) -> Option<Command<'a>> {
    let (from_codel, to_codel) = (*canvas.get_codel(from_coord), *canvas.get_codel(to_coord));
    if !from_codel.is_any_colour() || !to_codel.is_any_colour() {return None}
    let hue_diff =  from_codel.hue_difference(to_codel);
    let lightness_diff = from_codel.light_difference(to_codel);
    match (hue_diff, lightness_diff) {
        (Some(0),Some(0)) => None,
        (Some(0),Some(1)) => Some(Command::Push(&commands::push)),
        (Some(0),Some(2)) => Some(Command::StackOps(&commands::pop)),
        (Some(1),Some(0)) => Some(Command::StackOps(&commands::add)),
        (Some(1),Some(1)) => Some(Command::StackOps(&commands::subtract)),
        (Some(1),Some(2)) => Some(Command::StackOps(&commands::multiply)),
        (Some(2),Some(0)) => Some(Command::StackOps(&commands::divide)),
        (Some(2),Some(1)) => Some(Command::StackOps(&commands::modulo)),
        (Some(2),Some(2)) => Some(Command::StackOps(&commands::not)),
        (Some(3),Some(0)) => Some(Command::StackOps(&commands::greater)),
        (Some(3),Some(1)) => Some(Command::Interpreter(&commands::pointer)),
        (Some(3),Some(2)) => Some(Command::Interpreter(&commands::switch)),
        (Some(4),Some(0)) => Some(Command::StackOps(&commands::duplicate)),
        (Some(4),Some(1)) => Some(Command::StackOps(&commands::roll)),
        (Some(4),Some(2)) => Some(Command::InputNum(&commands::input_num)),
        (Some(5),Some(0)) => Some(Command::InputChar(&commands::input_char)),
        (Some(5),Some(1)) => Some(Command::OutputNum(&commands::output_num)),
        (Some(5),Some(2)) => Some(Command::OutputChar(&commands::output_char)),
        _ => None
}
}
