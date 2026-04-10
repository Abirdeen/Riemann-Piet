use std::io::{BufRead, Write};
use log;

use crate::canvas::{BlockSize, Canvas, Codel, CodelBlock, Coordinate};
use crate::surface::{Atlas, ChartIndex};

pub enum Command<'a> {
    Push((BlockSize, &'a dyn Fn(&mut Stack, BlockSize) -> ())),
    StackOps(&'a dyn Fn(&mut Stack) -> ()),
    Interpreter(&'a dyn Fn(&mut Interpreter) -> ()),
    InputNum(&'a dyn Fn(&mut Stack, i64) -> ()),
    InputChar(&'a dyn Fn(&mut Stack, char) -> ()),
    OutputNum(&'a dyn Fn(&mut Stack) -> i64),
    OutputChar(&'a dyn Fn(&mut Stack)-> Option<char>)
}

type Stack = Vec<i64>;

#[derive(Debug, Clone, Copy)]
pub enum DP {
    North,
    East,
    South,
    West
}
#[derive(Debug, Clone, Copy)]
pub enum CC {
    Left,
    Right
}
#[derive(Debug, Clone, Copy)]
pub enum InterpreterAspect {
    CC,
    DP,
    Both
}

enum StepState {
    StepWhite(Coordinate),
    StepColour(Coordinate),
    ChangeCanvas,
    HitBlack
}
enum MoveState<'a> {
    Continue(Option<Command<'a>>),
    ChangeInterpreterState,
    ChangeCanvas,
    Terminate,
    Error
}
pub enum CodeState<'a> {
    ModifyInterpreter(InterpreterAspect),
    Continue(Option<Command<'a>>),
    Terminate,
    ChangeCanvas(InterpreterAspect),
    Error
}
pub enum ProcessStateOutcome {
    Continue,
    CanvasChange(ChartIndex),
    ModifyInterpreter(InterpreterAspect),
    Terminate
}

impl<'a> From<MoveState<'a>> for CodeState<'a> {
    fn from(value: MoveState<'a>) -> Self {
        match value {
            MoveState::Continue(command) => CodeState::Continue(command),
            MoveState::ChangeCanvas => CodeState::ChangeCanvas(InterpreterAspect::Both),
            MoveState::ChangeInterpreterState => CodeState::ModifyInterpreter(InterpreterAspect::Both),
            MoveState::Terminate => CodeState::Terminate,
            MoveState::Error => CodeState::Error
        }
    }
}

pub struct Interpreter {
    stack: Stack,
    dp: DP,
    cc: CC,
    current_coordinate: Coordinate,
    reversed: bool
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {stack: Vec::new(), dp: DP::East, cc: CC::Left, current_coordinate: (0,0), reversed: false}
    }

    fn stack(&mut self) -> &mut Stack {
        &mut self.stack
    }
    pub fn dp(&self) -> DP {
        self.dp
    }
    fn cc(&self) -> CC {
        self.cc
    }
    fn reversed(&self) -> bool {
        self.reversed
    }
    pub fn current_coordinate(&self) -> Coordinate {
        self.current_coordinate
    }
    fn update_coordinate(&mut self, new_coordinate: Coordinate) {
        self.current_coordinate = new_coordinate
    }

    fn reverse(&mut self) {
        self.flip_cc();
        self.reversed = !self.reversed
    }
    fn flip_cc(&mut self) {
        match self.cc() {
            CC::Left => self.cc = CC::Right,
            CC::Right => self.cc = CC::Left
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
            DP::North => {self.dp = DP::East},
            DP::East => {self.dp = DP::South},
            DP::South => {self.dp = DP::West},
            DP::West => {self.dp = DP::North}
        }
        self.rotate_dp_right((n-1).rem_euclid(4));
    }
    fn rotate_dp_left(&mut self, n: i64) {
        if n.rem_euclid(4)==0 {
            return ()
        }
        match self.dp() {
            DP::North => {self.dp = DP::West},
            DP::West => {self.dp = DP::South},
            DP::South => {self.dp = DP::East},
            DP::East => {self.dp = DP::North}
        }
        self.rotate_dp_right((n-1).rem_euclid(4));
    }
    fn rotate_dp(&mut self, n: i64) {
        if self.reversed {
            self.rotate_dp_left(n);
        } else {
            self.rotate_dp_right(n);
        }
    }

    fn modify_interpreter(&mut self, interpreter_aspect: InterpreterAspect) {
        match interpreter_aspect {
            InterpreterAspect::CC => self.flip_cc(),
            InterpreterAspect::DP => self.rotate_dp(1),
            InterpreterAspect::Both => {
                self.flip_cc();
                self.rotate_dp(1);
            }
        }
    }

    fn execute_command(&mut self, command: Option<Command>) {
        match command {
            Some(Command::Push((block_size, func))) => {
                func(&mut self.stack(), block_size)
            },
            Some(Command::StackOps(func)) => {
                func(&mut self.stack())
            },
            Some(Command::Interpreter(func)) => {
                func(self)
            },
            Some(Command::InputChar(func)) => {
                print!("Input char: ");
                std::io::stdout().flush().expect("Failed to flush");

                let mut buf = String::new();
                std::io::stdin().lock().read_line(&mut buf).expect("Failed to read line");
                let char = buf.chars().nth(0).expect("No characters were read!");
                func(&mut self.stack(), char)
            },
            Some(Command::OutputChar(func)) => {
                let result = func(&mut self.stack());
                match result {
                    Some(character) => print!("{}", character),
                    None => ()
                }
            },
            Some(Command::InputNum(func)) => {
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
            Some(Command::OutputNum(func)) => {
                let output = func(&mut self.stack());
                print!("{}", output)
            },
            None => ()
        }
    }

    fn try_step_from_white(&self, canvas: &mut Canvas, current_coordinate: Coordinate) -> StepState {
        match self.next_coords(canvas, current_coordinate) {
            Some(next_coord) => {
                match canvas.get_codel(next_coord) {
                    Codel::White {..} => StepState::StepWhite(next_coord),
                    Codel::Colour(_) => StepState::StepColour(next_coord),
                    Codel::Black {..} => StepState::HitBlack
                }
            },
            None => return StepState::ChangeCanvas
        }        
    }
    fn try_move_through_white<'a>(&mut self, canvas: &mut Canvas) -> MoveState<'a> {
        let mut current_coord = self.current_coordinate();
        let mut visited: Vec<Coordinate> = Vec::new();
        loop {
            if visited.contains(&current_coord) {
                return MoveState::Terminate
            }

            match self.try_step_from_white(canvas, current_coord) {
                StepState::StepColour(coordinate) => {
                    self.update_coordinate(coordinate);
                    return MoveState::Continue(None)
                },
                StepState::StepWhite(coordinate) => {
                    visited.push(current_coord);
                    current_coord = coordinate
                },
                StepState::HitBlack => self.modify_interpreter(InterpreterAspect::Both),
                StepState::ChangeCanvas => {
                    self.update_coordinate(current_coord);
                    return MoveState::ChangeCanvas
                }
            };
        }
    }
    fn get_exit_coords(&self, block: &mut CodelBlock) -> Coordinate {
        match (self.dp(), self.cc()) {
            (DP::North, CC::Left) => block.northmost_west(),
            (DP::North, CC::Right) => block.northmost_east(),
            (DP::East, CC::Left) => block.eastmost_north(),
            (DP::East, CC::Right) => block.eastmost_south(),
            (DP::South, CC::Left) => block.southmost_east(),
            (DP::South, CC::Right) => block.southmost_west(),
            (DP::West, CC::Left) => block.westmost_south(),
            (DP::West, CC::Right) => block.westmost_north()
        }
    }
    fn next_coords(&self, canvas: &mut Canvas, coordinate: Coordinate) -> Option<Coordinate> {
        match self.dp() {
            DP::North => canvas.north(coordinate),
            DP::East => canvas.east(coordinate),
            DP::South => canvas.south(coordinate),
            DP::West => canvas.west(coordinate)
        }
    }
    fn try_move_from_colour<'a>(&mut self, canvas: &mut Canvas) -> MoveState<'a> {
        let result = canvas.get_block_from_coord(self.current_coordinate());
        match result {
            Some(block) => {
                let from_coord = self.get_exit_coords(block);
                let to_coord = match self.next_coords(canvas, from_coord) {
                    Some(coord) => {
                        coord
                    },
                    None => return MoveState::ChangeCanvas
                };
                if canvas.get_codel(to_coord).is_black() {
                    return MoveState::ChangeInterpreterState
                }
                log::debug!("Exit coordinate is {to_coord:?}");
                self.update_coordinate(to_coord);
                return MoveState::Continue(get_command(canvas, from_coord, to_coord))
            },
            None => return MoveState::Error
        }
    }

    fn get_next_state<'a>(&mut self, canvas: &mut Canvas, interpreter_aspect: InterpreterAspect) -> CodeState<'a> {
        match canvas.get_codel(self.current_coordinate()) {
            Codel::Black {..} => {
                log::error!("Interpreter ended up inside a black codel! This should be impossible!");
                return CodeState::Error
            },
            Codel::White {..} => {
                log::debug!("Interpreter stepping from a White block");
                return CodeState::from(self.try_move_through_white(canvas))
            },
            Codel::Colour(colour) => {
                let colour_name = colour.name();
                log::debug!("Interpreter stepping from a {colour_name} block");
                log::debug!("Codel chooser points {:?}, direction pointer points {:?}", self.cc(), self.dp());
                match self.try_move_from_colour(canvas) {
                    MoveState::ChangeInterpreterState => return CodeState::ModifyInterpreter(interpreter_aspect),
                    MoveState::ChangeCanvas => return CodeState::ChangeCanvas(interpreter_aspect),
                    other => return CodeState::from(other)
                }
            }
        }
    }
}

pub trait Interpretable {

    fn process_state(&mut self, interpreter: &mut Interpreter, state: CodeState, canvas_index: ChartIndex) -> ProcessStateOutcome;

    fn run(&mut self, interpreter: &mut Interpreter, max_heartbeats: i64);
}

impl Interpretable for Canvas {

    fn process_state(&mut self, interpreter: &mut Interpreter, state: CodeState, _: ChartIndex) -> ProcessStateOutcome {
        match state {
            CodeState::Continue(command) => {
                interpreter.execute_command(command);
                return ProcessStateOutcome::Continue
            },
            CodeState::ChangeCanvas(fallback) | CodeState::ModifyInterpreter(fallback) => {
                interpreter.modify_interpreter(fallback);
                log::debug!("Modified interpreter state");
                return ProcessStateOutcome::ModifyInterpreter(fallback)
            }
            CodeState::Terminate => {
                log::info!("Program terminated!");
                return ProcessStateOutcome::Terminate
            },
            CodeState::Error => {
                log::debug!("Code reached an unreachable state!");
                return ProcessStateOutcome::Terminate
            },
        }
    }

    fn run(&mut self, interpreter: &mut Interpreter, max_heartbeats: i64) {
        let mut heartbeats = 0;
        let mut dp_counter = 0;
        let mut interpreter_aspect = InterpreterAspect::CC;
        while heartbeats < max_heartbeats {
            let state = interpreter.get_next_state(self, interpreter_aspect);
            match self.process_state(interpreter, state, 0) {
                ProcessStateOutcome::Continue => {
                    dp_counter = 0;
                    interpreter_aspect = InterpreterAspect::CC;
                    heartbeats += 1;
                },
                ProcessStateOutcome::ModifyInterpreter(InterpreterAspect::CC) => {
                    interpreter_aspect = InterpreterAspect::DP;
                },
                ProcessStateOutcome::ModifyInterpreter(_) => {
                    dp_counter +=1;
                    if dp_counter > 3 {
                        log::debug!("Interpreter could not progress");
                        break
                    }
                    interpreter_aspect = InterpreterAspect::CC;
                },
                ProcessStateOutcome::Terminate | ProcessStateOutcome::CanvasChange(_) => {
                    break
                }
            }
        }
        log::info!("Program terminated after {heartbeats} steps")        
    }
}

impl Interpretable for Atlas {
    fn process_state(&mut self, interpreter: &mut Interpreter, state: CodeState, index: ChartIndex) -> ProcessStateOutcome {
        match state {
            CodeState::Continue(command) => {
                interpreter.execute_command(command);
                return ProcessStateOutcome::Continue
            },
            CodeState::ChangeCanvas(fallback) => {
                let transition_map = self.transition_map();
                match transition_map(interpreter, index) {
                    Some((new_index, reverse)) => {
                        match self.transition_coords(interpreter, index, new_index, reverse) {
                            Some(coordinate) => {
                                interpreter.update_coordinate(coordinate);
                                log::debug!("Changed canvas");
                                if reverse {
                                    interpreter.reverse();
                                }
                                return ProcessStateOutcome::CanvasChange(new_index)
                            },
                            None => ()
                        };
                    },
                    None => ()
                };
                interpreter.modify_interpreter(fallback);
                log::debug!("Modified interpreter state");
                return ProcessStateOutcome::ModifyInterpreter(fallback)
            },
            CodeState::ModifyInterpreter(fallback) => {
                interpreter.modify_interpreter(fallback);
                log::debug!("Modified interpreter state");
                return ProcessStateOutcome::ModifyInterpreter(fallback)
            }
            CodeState::Terminate => {
                log::info!("Program terminated!");
                return ProcessStateOutcome::Terminate
            },
            CodeState::Error => {
                log::debug!("Code reached an unreachable state!");
                return ProcessStateOutcome::Terminate
            },
        }
    }

    fn run(&mut self, interpreter: &mut Interpreter, max_heartbeats: i64) {
        let mut heartbeats = 0;
        let mut dp_counter = 0;
        let mut interpreter_aspect = InterpreterAspect::CC;
        let mut current_chart_index = self.origin().index();
        while heartbeats < max_heartbeats {
            let current_chart = self.chart_from_index(current_chart_index).expect("Incorrectly updated chart");
            let state = interpreter.get_next_state(current_chart.canvas(), interpreter_aspect);
            match self.process_state(interpreter, state, current_chart_index) {
                ProcessStateOutcome::Continue => {
                    dp_counter = 0;
                    interpreter_aspect = InterpreterAspect::CC;
                    heartbeats += 1;
                },
                ProcessStateOutcome::CanvasChange(i) => {
                    current_chart_index = i;
                },
                ProcessStateOutcome::ModifyInterpreter(InterpreterAspect::CC) => {
                    interpreter_aspect = InterpreterAspect::DP;
                },
                ProcessStateOutcome::ModifyInterpreter(_) => {
                    dp_counter +=1;
                    if dp_counter > 3 {
                        log::debug!("Interpreter could not progress");
                        break
                    }
                    interpreter_aspect = InterpreterAspect::CC;
                },
                ProcessStateOutcome::Terminate => {
                    break
                }
            }
        }
        log::info!("Program terminated after {heartbeats} steps")
    }
}

mod commands {
    use std::collections::VecDeque;

    use crate::{canvas::BlockSize, interpreter::{Interpreter, Stack}};

    // Pushes the number of codels in the previous color block onto the stack.
    pub fn push(stack: &mut Stack, value: BlockSize) {
        log::debug!("Pushed {value}");
        stack.push(value as i64);
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
        interpreter.rotate_dp(n);
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
    let block_size = canvas.get_block_from_coord(from_coord)?.size();
    if !from_codel.is_any_colour() || !to_codel.is_any_colour() {return None}
    let hue_diff =  from_codel.hue_difference(to_codel);
    let lightness_diff = from_codel.light_difference(to_codel);
    match (hue_diff, lightness_diff) {
        (Some(0),Some(0)) => None,
        (Some(0),Some(1)) => Some(Command::Push((block_size, &commands::push))),
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
