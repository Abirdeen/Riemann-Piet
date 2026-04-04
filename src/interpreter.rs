use std::io::{BufRead, Write};
use log;

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

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {stack: Vec::new(), dp: dp::EAST, cc: cc::LEFT, current_coordinate: (0,0)}
    }

    fn stack(&mut self) -> &mut Stack {
        &mut self.stack
    }
    fn dp(&self) -> DirectionalPointer {
        self.dp
    }
    fn cc(&self) -> CodelChooser {
        self.cc
    }
    fn current_coordinate(&self) -> Coordinate {
        self.current_coordinate
    }
    fn update_coordinate(&mut self, new_coordinate: Coordinate) {
        self.current_coordinate = new_coordinate
    }

    fn flip_cc(&mut self) {
        match self.cc() {
            cc::LEFT => self.cc = cc::RIGHT,
            cc::RIGHT => self.cc = cc::LEFT,
            _ => ()
        }        
    }
    fn flip_n_cc(&mut self, n: i64) {
        if n%2==0 {return ()}
        self.flip_cc();
    }
    fn rotate_dp_right(&mut self, n: i64) {
        if n.rem_euclid(4)==0 {
            return ()
        }
        match self.dp() {
            dp::NORTH => {self.dp = dp::EAST},
            dp::EAST => {self.dp = dp::SOUTH},
            dp::SOUTH => {self.dp = dp::WEST},
            dp::WEST => {self.dp = dp::NORTH},
            _ => ()
        }
        self.rotate_dp_right((n-1).rem_euclid(4));
    }

    fn execute_command(&mut self, canvas: &mut Canvas, command: Command) {
        match command {
            Command::Push(func) => {
                let result = canvas.get_block_from_coord(self.current_coordinate());
                match result {
                    Some(block) => func(&mut self.stack(), block.size() as i64),
                    None => ()
                }
            },
            Command::StackOps(func) => {
                func(&mut self.stack())
            },
            Command::Interpreter(func) => {
                func(self)
            },
            Command::InputChar(func) => {
                print!("Input char: ");
                std::io::stdout().flush().expect("Failed to flush");

                let mut buf = String::new();
                std::io::stdin().lock().read_line(&mut buf).expect("Failed to read line");
                let char = buf.chars().nth(0).expect("No characters were read!");
                func(&mut self.stack(), char)
            },
            Command::OutputChar(func) => {
                let result = func(&mut self.stack());
                match result {
                    Some(character) => print!("{}", character),
                    None => ()
                }
            },
            Command::InputNum(func) => {
                print!("Input num: ");
                std::io::stdout().flush().expect("Failed to flush");
                let mut buf = String::new();
                std::io::stdin().lock().read_line(&mut buf).expect("Failed to read line");
                let input = buf.trim();
                match input.parse::<i64>() {
                    Ok(val) => func(&mut self.stack(), val),
                    Err(_) => ()
                }
            },
            Command::OutputNum(func) => {
                let output = func(&mut self.stack());
                print!("{}", output)
            }
        }
    }

    fn try_step_from_white(&self, canvas: &mut Canvas, current_coordinate: Coordinate) -> Option<(Coordinate, bool)> {
        match self.next_coords(canvas, current_coordinate) {
            Some(next_coord) => {
                match canvas.get_codel(next_coord) {
                    Codel::White {..} => Some((next_coord, false)),
                    Codel::Colour(_) => Some((next_coord, true)),
                    Codel::Black {..} => None
                }
            },
            None => return None
        }        
    }

    fn try_move_through_white(&mut self, canvas: &mut Canvas) -> bool {
        let mut current_coord = self.current_coordinate();
        let mut visited: Vec<Coordinate> = Vec::new();
        loop {
            if visited.contains(&current_coord) {
                return false
            }
            visited.push(current_coord);

            match self.try_step_from_white(canvas, current_coord) {
                Some((coordinate, true)) => {
                    self.update_coordinate(coordinate);
                    return true
                },
                Some((coordinate, false)) => current_coord = coordinate,
                None => {self.flip_cc(); self.rotate_dp_right(1);}
            };
        }
    }
    fn get_exit_coords(&self, block: &mut CodelBlock) -> Coordinate {
        match (self.dp(), self.cc()) {
            (dp::NORTH, cc::LEFT) => block.northmost_west(),
            (dp::NORTH, cc::RIGHT) => block.northmost_east(),
            (dp::EAST, cc::LEFT) => block.eastmost_north(),
            (dp::EAST, cc::RIGHT) => block.eastmost_south(),
            (dp::SOUTH, cc::LEFT) => block.southmost_east(),
            (dp::SOUTH, cc::RIGHT) => block.southmost_west(),
            (dp::WEST, cc::LEFT) => block.westmost_south(),
            (dp::WEST, cc::RIGHT) => block.westmost_north(),
            _ => panic!()
        }
    }
    fn next_coords(&self, canvas: &mut Canvas, coordinate: Coordinate) -> Option<Coordinate> {
        match self.dp() {
            dp::NORTH => {if !canvas.is_northmost(coordinate) {Some(canvas.north(coordinate))} else {None}},
            dp::EAST => {if !canvas.is_eastmost(coordinate) {Some(canvas.east(coordinate))} else {None}},
            dp::SOUTH => {if !canvas.is_southmost(coordinate) {Some(canvas.south(coordinate))} else {None}},
            dp::WEST => {if !canvas.is_westmost(coordinate) {Some(canvas.west(coordinate))} else {None}},
            _ => None
        }
    }
    fn try_move_from_colour(&mut self, canvas: &mut Canvas) -> bool {
        let result = canvas.get_block_from_coord(self.current_coordinate());
        match result {
            Some(block) => {
                let from_coord = self.get_exit_coords(block);
                let to_coord = match self.next_coords(canvas, from_coord) {
                    Some(coord) => {
                        log::debug!("Exit coordinate is {coord:?}");
                        coord
                    },
                    // Will only be none if you're at the edge of a canvas; in future, this will try to move to a new canvas.
                    None => return false
                };
                let next_codel = canvas.get_codel(to_coord);
                if next_codel.is_black() {
                    return false
                }
                match get_command(canvas, from_coord, to_coord) {
                    Some(command) => self.execute_command(canvas, command),
                    None => ()
                };
                self.update_coordinate(to_coord);
                return true
            },
            None => return false
        }
    }

    fn step(&mut self, canvas: &mut Canvas) -> bool {
        match canvas.get_codel(self.current_coordinate()) {
            Codel::Black {..} => {
                log::error!("Interpreter ended up inside a black codel! This should be impossible!");
                return false
            },
            Codel::White {..} => {
                log::debug!("Interpreter stepping from a white block");
                return self.try_move_through_white(canvas)
            },
            Codel::Colour(colour) => {
                let colour_name = colour.name();
                log::debug!("Interpreter stepping from a {colour_name} block");
                log::debug!("Codel chooser points {}, direction pointer points {}", self.cc(), self.dp());
                for _ in 1..4 {
                    if self.try_move_from_colour(canvas) {
                        return true
                    };
                    self.flip_cc();
                    if self.try_move_from_colour(canvas) {
                        return true
                    }
                    self.rotate_dp_right(1);
                    log::debug!("Codel chooser points {}, direction pointer points {}", self.cc(), self.dp());
                }
                return false
            }
        }
    }

    pub fn run(&mut self, canvas: &mut Canvas, max_heartbeats: i64) {
        let mut continue_program = true;
        let mut heartbeats = 0_i64;
        while continue_program && heartbeats < max_heartbeats {
            continue_program = self.step(canvas);
            heartbeats += 1;
        }
        log::info!("Program terminated after {heartbeats} steps")
    }
}

mod commands {
    use std::collections::VecDeque;

    use crate::interpreter::{Interpreter, Stack};

    // Pushes the number of codels in the previous color block onto the stack.
    pub fn push(stack: &mut Stack, value: i64) {
        log::debug!("Pushed {value}");
        stack.push(value);
        log::debug!("Resulting stack is {stack:?}")
    }
    // Pops the top value off the stack.
    pub fn pop(stack: &mut Stack) -> () {
        log::debug!("Popped from stack");
        stack.pop();
        log::debug!("Resulting stack is {stack:?}")
    }
    // Pops the top two values off the stack, adds them up, and pushes the sum back onto the stack.
    pub fn add(stack: &mut Stack) -> () {
        let x = stack.pop().unwrap_or(0);
        let y = stack.pop().unwrap_or(0);
        log::debug!("Added {x} + {y}");
        stack.push(x+y);
        log::debug!("Resulting stack is {stack:?}")
    }
    // Pops the top two values off the stack, subtracts the top value from the second-top value, and pushes the difference back onto the stack. Note that if the top value is X and the next value Y, this means that Y - X will be pushed, not X - Y.
    pub fn subtract(stack: &mut Stack) -> () {
        let x = stack.pop().unwrap_or(0);
        let y = stack.pop().unwrap_or(0);
        log::debug!("Subtracted {y} - {x}");
        stack.push(y-x);
        log::debug!("Resulting stack is {stack:?}")
    }
    // Pops the top two values off the stack, multiplies them together, and pushes the product back onto the stack.
    pub fn multiply(stack: &mut Stack) -> () {
        let x = stack.pop().unwrap_or(0);
        let y = stack.pop().unwrap_or(0);
        log::debug!("Multiplied {x} * {y}");
        stack.push(x*y);
        log::debug!("Resulting stack is {stack:?}")
    }
    // Pops the top two values off the stack, performs integer division (Python equivalent of //) on the second-top value divided by the top value, and pushes the quotient back onto the stack. This has the same X/Y property as subtraction.
    pub fn divide(stack: &mut Stack) -> () {
        let x = stack.pop().unwrap_or(1);
        let y = stack.pop().unwrap_or(0);
        log::debug!("Divided {y} / {x}");
        stack.push(y/x);
        log::debug!("Resulting stack is {stack:?}")
    }
    // Pops the top two values off the stack, divided the second-top value by the top value, and pushes the remainder back onto the stack. This has the same X/Y property as subtraction.
    pub fn modulo(stack: &mut Stack) -> () {
        let x = stack.pop().unwrap_or(1);
        let y = stack.pop().unwrap_or(0);
        log::debug!("Took {y} mod {x}");
        stack.push(y.rem_euclid(x));
        log::debug!("Resulting stack is {stack:?}")
    }
    // Pops the top value off the stack. If the value is 0, it pushes 1 onto the stack. Otherwise, it pushes 0.
    pub fn not(stack: &mut Stack) -> () {
        let x = stack.pop().unwrap_or(0);
        log::debug!("Found ¬{x}");
        stack.push(if x==0 {1} else {0});
        log::debug!("Resulting stack is {stack:?}")
    }
    // Pops the top two values off the stack. If the second-top value is greater than the top value, it pushes 1 onto the stack. Otherwise, it pushes 0. This has the same X/Y property as subtraction.
    pub fn greater(stack: &mut Stack) -> () {
        let x = stack.pop().unwrap_or(1);
        let y = stack.pop().unwrap_or(0);
        log::debug!("Tested {x} < {y}");
        stack.push(if x<y {1} else {0});
        log::debug!("Resulting stack is {stack:?}")
    }
    // Pushes a copy of the top value onto the stack.
    pub fn duplicate(stack: &mut Stack) -> () {
        let x = stack.pop().unwrap_or(0);
        log::debug!("Duplicated {x}");
        stack.push(x);
        stack.push(x);
        log::debug!("Resulting stack is {stack:?}")
    }
    // Pops the top two values off the stack, and then rotates the top Y values on the stack up by X, wrapping values that pass the top around to the bottom of the rolled portion, where X is the first value popped (top of the stack), and Y is the second value popped (second on the stack). (Example: If the stack is currently 1,2,3, with 3 at the top, and then you push 3 and then 1, and then roll, the new stack is 3,1,2.)
    pub fn roll(stack: &mut Stack) -> () {
        let x = stack.pop().unwrap_or(0);
        let y = stack.pop().unwrap_or(0).min(stack.len() as i64);

        if y==0 {
            log::debug!("Rolled zero values");
            return ()
        }

        log::debug!("Rolled {y} values by {x}");

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
        log::debug!("Resulting stack is {stack:?}")
    }
    // Pops the top value off the stack, then rotates the DP one step clockwise that many times (anti-clockwise if the value is negative).
    pub fn pointer(interpreter: &mut Interpreter) -> () {
        let stack = interpreter.stack();
        let n = stack.pop().unwrap_or(0);
        log::debug!("Rotated direction pointer by {n}");
        interpreter.rotate_dp_right(n);
    }
    // Pops the top value off the stack, then switches the state of the CC that many times (absolute value if the value is negative).
    pub fn switch(interpreter: &mut Interpreter) -> () {
        let stack = interpreter.stack();
        let n = stack.pop().unwrap_or(0);
        log::debug!("Flipped codel chooser by {n}");
        interpreter.flip_n_cc(n);
    }
    // Takes an input, either as a character or a number. If the input is a number, that value is pushed onto the stack. If it's a character, its Unicode value is pushed onto the stack.
    pub fn input_num(stack: &mut Stack, input: i64) -> () {
        log::debug!("Took {input} as input");
        stack.push(input);
        log::debug!("Resulting stack is {stack:?}")
    }
    pub fn input_char(stack: &mut Stack, input: char) -> () {
        log::debug!("Took {input} as input");
        stack.push(input as i64);
        log::debug!("Resulting stack is {stack:?}")
        }
    // Pops the top value off the stack. If a number should be printed, the value itself will be printed. If a character should be printed, then its corresponding Unicode character will be printed.
    pub fn output_num(stack: &mut Stack) -> i64 {
        let output = stack.pop().unwrap_or(0);
        log::debug!("Output {output}");
        log::debug!("Resulting stack is {stack:?}");
        return output
    }
    pub fn output_char(stack: &mut Stack) -> Option<char> {
        let char_int = stack.pop().unwrap_or(0);
        let output = char::from_u32(char_int as u32);
        log::debug!("Output {output:?}");
        log::debug!("Resulting stack is {stack:?}");
        return output
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
