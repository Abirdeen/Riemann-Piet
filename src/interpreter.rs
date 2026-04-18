use std::io::{BufRead, Write};
use log;

use crate::canvas::{BlockSize, Canvas, Codel, CodelBlock, Coordinate};
use crate::surface::{Atlas, ChartIndex};
use crate::palette::PietColour;

#[derive(Clone, Copy)]
pub enum PietCommand {
    Push(BlockSize),
    Pop,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Not,
    Greater,
    Pointer,
    Switch,
    Duplicate,
    Roll,
    InputNum,
    InputChar,
    OutputNum,
    OutputChar
}

type PietStack = Vec<i64>;

#[derive(Debug, Clone, Copy, PartialEq)]
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
pub enum PointerAspect {
    CC,
    DP,
    Both
}

enum StepState {
    StepWhite,
    StepColour,
    HitBlack
}
enum MoveState {
    Continue(Option<PietCommand>),
    ChangePointerState,
    Terminate,
    Error
}
pub enum CodeState {
    ModifyPointer(PointerAspect),
    Continue(Option<PietCommand>),
    Terminate,
    Error
}
pub enum ProcessStateOutcome {
    Continue,
    ModifyPointer(PointerAspect),
    Terminate
}

impl<'a> From<MoveState> for CodeState {
    fn from(value: MoveState) -> Self {
        match value {
            MoveState::Continue(command) => CodeState::Continue(command),
            MoveState::ChangePointerState => CodeState::ModifyPointer(PointerAspect::Both),
            MoveState::Terminate => CodeState::Terminate,
            MoveState::Error => CodeState::Error
        }
    }
}

pub struct Interpreter {
    stack: PietStack,
    dp: DP,
    cc: CC,
    current_coordinate: Coordinate,
    current_chart_index: ChartIndex,
    reversed: bool
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {stack: Vec::new(), dp: DP::East, cc: CC::Left, current_coordinate: (0,0), current_chart_index: 0, reversed: false}
    }
    pub fn from_data(dp: DP, cc: CC, current_coordinate: Coordinate, current_chart_index: ChartIndex, reversed: bool) -> Interpreter {
        Interpreter { stack: Vec::new(), dp, cc, current_coordinate, current_chart_index, reversed }
    }

    fn stack(&mut self) -> &mut PietStack {
        &mut self.stack
    }

    pub fn dp(&self) -> DP {
        self.dp
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

    pub fn cc(&self) -> CC {
        self.cc
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

    pub fn modify_pointer(&mut self, pointer_aspect: PointerAspect) {
        match pointer_aspect {
            PointerAspect::CC => self.flip_cc(),
            PointerAspect::DP => self.rotate_dp(1),
            PointerAspect::Both => {
                self.flip_cc();
                self.rotate_dp(1);
            }
        }
    }

    fn reverse(&mut self) {
        self.flip_cc();
        self.reversed = !self.reversed
    }

    pub fn current_coordinate(&self) -> Coordinate {
        self.current_coordinate
    }
    fn update_coordinate(&mut self, new_coordinate: Coordinate) {
        self.current_coordinate = new_coordinate
    }
    pub fn current_chart_index(&self) -> ChartIndex {
        self.current_chart_index
    }
    fn update_chart_index(&mut self, new_index: ChartIndex) {
        self.current_chart_index = new_index
    }
}

pub trait Interpretable {
    fn current_canvas(&self, interpreter: &Interpreter) -> &Canvas;

    fn canvas_from_index(&self, index: ChartIndex) -> Option<&Canvas>;

    fn move_coord(&self, interpreter: &mut Interpreter, coordinate: Coordinate) -> bool;

    fn is_white(&self, interpreter: &Interpreter, coordinate: Coordinate) -> bool {
        self.current_canvas(interpreter).is_colour(coordinate, PietColour::White)
    }
}

impl Interpretable for Canvas {
    
    fn current_canvas(&self, _: &Interpreter) -> &Canvas {
        self
    }

    fn canvas_from_index(&self, _: ChartIndex) -> Option<&Canvas> {
        Some(self)
    }

    fn move_coord(&self, interpreter: &mut Interpreter, coordinate: Coordinate) -> bool {
        let from_boundary = match interpreter.dp() {
            DP::North => self.is_northmost(coordinate),
            DP::East => self.is_eastmost(coordinate),
            DP::South => self.is_southmost(coordinate),
            DP::West => self.is_westmost(coordinate)
        };
        if from_boundary {
            return false
        }

        let (x,y) = coordinate;
        let new_coord = match interpreter.dp() {
            DP::North => (x,y-1),
            DP::East => (x+1,y),
            DP::South => (x,y+1),
            DP::West => (x-1,y)
        };
        if self.is_colour(new_coord, PietColour::Black) {
            return false
        }
        interpreter.update_coordinate(new_coord);
        return true
    }
}

impl<'a> Interpretable for Atlas<'a> {

    fn current_canvas(&self, interpreter: &Interpreter) -> &Canvas {
        self.chart(interpreter.current_chart_index()).expect("Improperly updated chart index").canvas()
    }

    fn canvas_from_index(&self, index: ChartIndex) -> Option<&Canvas> {
        Some(self.chart(index)?.canvas())
    }

    fn move_coord(&self, interpreter: &mut Interpreter, coordinate: Coordinate) -> bool {
        let current_canvas = self.current_canvas(interpreter);
        let from_boundary = match interpreter.dp() {
            DP::North => current_canvas.is_northmost(coordinate),
            DP::East => current_canvas.is_eastmost(coordinate),
            DP::South => current_canvas.is_southmost(coordinate),
            DP::West => current_canvas.is_westmost(coordinate)
        };
        if from_boundary {
            let from_index = interpreter.current_chart_index();
            match self.transition_map()(interpreter, from_index) {
                Some((to_index,reverse)) => {
                    match self.transition_coord(interpreter, from_index, to_index, reverse) {
                        Some(new_coord) => {
                            if !self.chart(to_index).expect("Mapped to nonexistent chart").canvas().is_colour(new_coord, PietColour::Black) {
                                interpreter.update_chart_index(to_index);
                                interpreter.update_coordinate(new_coord);
                                if reverse {
                                    interpreter.reverse();
                                }
                                return true
                            }
                        }, // extremely messy! Needs rewrite.
                        None => return false,
                    }
                },
                None => return false
            }

        };
        let (x,y) = coordinate;
        let new_coord: Coordinate = match interpreter.dp() {
            DP::North => (x,y-1),
            DP::East => (x+1,y),
            DP::South => (x,y+1),
            DP::West => (x-1,y)
        };
        if current_canvas.is_colour(new_coord, PietColour::Black) {
            return false
        }
        interpreter.update_coordinate(new_coord);
        return true 
    }
}

impl Interpreter {
    fn execute_command(&mut self, maybe_command: Option<PietCommand>) {
        let stack = self.stack();
        match maybe_command {
            Some(piet_command) => {
                match piet_command {
                    PietCommand::Push(block_size) => {
                        commands::push(stack, block_size)
                    },
                    PietCommand::Pop => {
                        commands::pop(stack);
                    },
                    PietCommand::Add => {
                        commands::add(stack);
                    },
                    PietCommand::Subtract => {
                        commands::subtract(stack);
                    }
                    PietCommand::Multiply => {
                        commands::multiply(stack);
                    },
                    PietCommand::Divide => {
                        commands::divide(stack);
                    },
                    PietCommand::Modulo => {
                        commands::modulo(stack);
                    },
                    PietCommand::Not => {
                        commands::not(stack);
                    },
                    PietCommand::Greater => {
                        commands::greater(stack);
                    },
                    PietCommand::Pointer => {
                        commands::pointer(self);
                    },
                    PietCommand::Switch => {
                        commands::switch(self);
                    },
                    PietCommand::Duplicate => {
                        commands::duplicate(stack);
                    },
                    PietCommand::Roll => {
                        commands::roll(stack);
                    },
                    PietCommand::InputChar => {
                        print!("Input char: ");
                        std::io::stdout().flush().expect("Failed to flush");

                        let mut buf = String::new();
                        std::io::stdin().lock().read_line(&mut buf).expect("Failed to read line");
                        let char = buf.chars().nth(0).expect("No characters were read!");
                        commands::input_char(stack, char)
                    },
                    PietCommand::OutputChar => {
                        let result = commands::output_char(&mut self.stack());
                        match result {
                            Some(character) => print!("{}", character),
                            None => ()
                        }
                    },
                    PietCommand::InputNum => {
                        print!("Input num: ");
                        std::io::stdout().flush().expect("Failed to flush");
                        let mut buf = String::new();
                        std::io::stdin().lock().read_line(&mut buf).expect("Failed to read line");
                        let input = buf.trim();
                        match input.parse::<i64>() {
                            Ok(val) => commands::input_num(&mut self.stack(), val),
                            Err(_) => ()
                        }
                    },
                    PietCommand::OutputNum => {
                        let output = commands::output_num(&mut self.stack());
                        print!("{}", output)
                    },
                }
            },
            None => ()
        }
    }

    fn try_step_from_white(&mut self, artwork: &impl Interpretable, current_coordinate: Coordinate) -> StepState {
        if !artwork.move_coord(self, current_coordinate) {
            return StepState::HitBlack
        }

        if artwork.is_white(self, self.current_coordinate()) {
            return StepState::StepWhite
        }
        return StepState::StepColour  
    }
    fn try_move_through_white(&mut self, artwork: &impl Interpretable) -> MoveState {
        let mut current_chart: ChartIndex = self.current_chart_index();
        let mut current_coord: Coordinate = self.current_coordinate();
        let mut visited: Vec<(ChartIndex, Coordinate, DP)> = Vec::new();
        loop {
            if visited.contains(&(current_chart, current_coord, self.dp())) {
                return MoveState::Terminate
            }

            match self.try_step_from_white(artwork, current_coord) {
                StepState::StepColour => {
                    return MoveState::Continue(None)
                },
                StepState::StepWhite => {
                    visited.push((current_chart, current_coord, self.dp()));
                    current_chart = self.current_chart_index();
                    current_coord = self.current_coordinate();
                },
                StepState::HitBlack => self.modify_pointer(PointerAspect::Both)
            };
        }
    }
    fn get_exit_coords(&self, block: &CodelBlock) -> Coordinate {
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

    fn try_move_from_colour(&mut self, artwork: &impl Interpretable) -> MoveState {
        let result = artwork.current_canvas(self).get_block(self.current_coordinate());
        match result {
            Some(block) => {
                let from_coord = self.get_exit_coords(block);
                let from_index = self.current_chart_index();
                if !artwork.move_coord(self, from_coord) {
                    return MoveState::ChangePointerState
                }
                let to_coord = self.current_coordinate();
                let to_index = self.current_chart_index();

                return MoveState::Continue(get_command(artwork, from_index, from_coord, to_index, to_coord))
            },
            None => return MoveState::Error
        }
    }

    pub fn get_next_state(&mut self, artwork: &impl Interpretable, pointer_aspect: PointerAspect) -> CodeState {
        match artwork.current_canvas(self).get_codel(self.current_coordinate()) {
            Codel::Black {..} => {
                log::error!("Interpreter ended up inside a black codel! This should be impossible!");
                return CodeState::Error
            },
            Codel::White {..} => {
                log::debug!("Interpreter stepping from a White block");
                return CodeState::from(self.try_move_through_white(artwork))
            },
            Codel::Colour(colour) => {
                let colour_name = colour.name();
                log::debug!("Interpreter stepping from a {:?} block", colour_name);
                log::debug!("Codel chooser points {:?}, direction pointer points {:?}", self.cc(), self.dp());
                match self.try_move_from_colour(artwork) {
                    MoveState::ChangePointerState => return CodeState::ModifyPointer(pointer_aspect),
                    other => return CodeState::from(other)
                }
            }
        }
    }

    fn process_state(&mut self, state: CodeState) -> ProcessStateOutcome {
        match state {
            CodeState::Continue(command) => {
                self.execute_command(command);
                return ProcessStateOutcome::Continue
            },
            CodeState::ModifyPointer(fallback) => {
                self.modify_pointer(fallback);
                log::debug!("Modified interpreter state");
                return ProcessStateOutcome::ModifyPointer(fallback)
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

    pub fn run(&mut self, artwork: &impl Interpretable, max_heartbeats: i64) {
        let mut heartbeats = 0;
        let mut dp_counter = 0;
        let mut pointer_aspect = PointerAspect::CC;
        while heartbeats < max_heartbeats {
            let state = self.get_next_state(artwork, pointer_aspect);
            match self.process_state(state) {
                ProcessStateOutcome::Continue => {
                    dp_counter = 0;
                    pointer_aspect = PointerAspect::CC;
                    heartbeats += 1;
                },
                ProcessStateOutcome::ModifyPointer(PointerAspect::CC) => {
                    pointer_aspect = PointerAspect::DP;
                },
                ProcessStateOutcome::ModifyPointer(_) => {
                    dp_counter +=1;
                    if dp_counter > 3 {
                        log::debug!("Interpreter could not progress");
                        break
                    }
                    pointer_aspect = PointerAspect::CC;
                },
                ProcessStateOutcome::Terminate => {
                    break
                }
            }
        }
        log::info!("Program terminated after {heartbeats} steps")
    }
}

pub mod commands {
    use std::collections::VecDeque;

    use crate::{canvas::BlockSize, interpreter::{Interpreter, PietStack}};

    // Pushes the number of codels in the previous color block onto the stack.
    pub fn push(stack: &mut PietStack, value: BlockSize) {
        log::debug!("Pushed {value}");
        stack.push(value as i64);
        log::debug!("Resulting stack is {stack:?}")
    }
    // Pops the top value off the stack.
    pub fn pop(stack: &mut PietStack) -> () {
        log::debug!("Popped from stack");
        stack.pop();
        log::debug!("Resulting stack is {stack:?}")
    }
    // Pops the top two values off the stack, adds them up, and pushes the sum back onto the stack.
    pub fn add(stack: &mut PietStack) -> () {
        let x = stack.pop().unwrap_or(0);
        let y = stack.pop().unwrap_or(0);
        log::debug!("Added {x} + {y}");
        stack.push(x+y);
        log::debug!("Resulting stack is {stack:?}")
    }
    // Pops the top two values off the stack, subtracts the top value from the second-top value, and pushes the difference back onto the stack. Note that if the top value is X and the next value Y, this means that Y - X will be pushed, not X - Y.
    pub fn subtract(stack: &mut PietStack) -> () {
        let x = stack.pop().unwrap_or(0);
        let y = stack.pop().unwrap_or(0);
        log::debug!("Subtracted {y} - {x}");
        stack.push(y-x);
        log::debug!("Resulting stack is {stack:?}")
    }
    // Pops the top two values off the stack, multiplies them together, and pushes the product back onto the stack.
    pub fn multiply(stack: &mut PietStack) -> () {
        let x = stack.pop().unwrap_or(0);
        let y = stack.pop().unwrap_or(0);
        log::debug!("Multiplied {x} * {y}");
        stack.push(x*y);
        log::debug!("Resulting stack is {stack:?}")
    }
    // Pops the top two values off the stack, performs integer division (Python equivalent of //) on the second-top value divided by the top value, and pushes the quotient back onto the stack. This has the same X/Y property as subtraction.
    pub fn divide(stack: &mut PietStack) -> () {
        let x = stack.pop().unwrap_or(1);
        let y = stack.pop().unwrap_or(0);
        log::debug!("Divided {y} / {x}");
        stack.push(y/x);
        log::debug!("Resulting stack is {stack:?}")
    }
    // Pops the top two values off the stack, divided the second-top value by the top value, and pushes the remainder back onto the stack. This has the same X/Y property as subtraction.
    pub fn modulo(stack: &mut PietStack) -> () {
        let x = stack.pop().unwrap_or(1);
        let y = stack.pop().unwrap_or(0);
        log::debug!("Took {y} mod {x}");
        stack.push(y.rem_euclid(x));
        log::debug!("Resulting stack is {stack:?}")
    }
    // Pops the top value off the stack. If the value is 0, it pushes 1 onto the stack. Otherwise, it pushes 0.
    pub fn not(stack: &mut PietStack) -> () {
        let x = stack.pop().unwrap_or(0);
        log::debug!("Found ¬{x}");
        stack.push(if x==0 {1} else {0});
        log::debug!("Resulting stack is {stack:?}")
    }
    // Pops the top two values off the stack. If the second-top value is greater than the top value, it pushes 1 onto the stack. Otherwise, it pushes 0. This has the same X/Y property as subtraction.
    pub fn greater(stack: &mut PietStack) -> () {
        let x = stack.pop().unwrap_or(1);
        let y = stack.pop().unwrap_or(0);
        log::debug!("Tested {x} < {y}");
        stack.push(if x<y {1} else {0});
        log::debug!("Resulting stack is {stack:?}")
    }
    // Pushes a copy of the top value onto the stack.
    pub fn duplicate(stack: &mut PietStack) -> () {
        let x = stack.pop().unwrap_or(0);
        log::debug!("Duplicated {x}");
        stack.push(x);
        stack.push(x);
        log::debug!("Resulting stack is {stack:?}")
    }
    // Pops the top two values off the stack, and then rotates the top Y values on the stack up by X, wrapping values that pass the top around to the bottom of the rolled portion, where X is the first value popped (top of the stack), and Y is the second value popped (second on the stack). (Example: If the stack is currently 1,2,3, with 3 at the top, and then you push 3 and then 1, and then roll, the new stack is 3,1,2.)
    pub fn roll(stack: &mut PietStack) -> () {
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
    pub fn input_num(stack: &mut PietStack, input: i64) -> () {
        log::debug!("Took {input} as input");
        stack.push(input);
        log::debug!("Resulting stack is {stack:?}")
    }
    pub fn input_char(stack: &mut PietStack, input: char) -> () {
        log::debug!("Took {input} as input");
        stack.push(input as i64);
        log::debug!("Resulting stack is {stack:?}")
        }
    // Pops the top value off the stack. If a number should be printed, the value itself will be printed. If a character should be printed, then its corresponding Unicode character will be printed.
    pub fn output_num(stack: &mut PietStack) -> i64 {
        let output = stack.pop().unwrap_or(0);
        log::debug!("Output {output}");
        log::debug!("Resulting stack is {stack:?}");
        return output
    }
    pub fn output_char(stack: &mut PietStack) -> Option<char> {
        let char_int = stack.pop().unwrap_or(0);
        let output = char::from_u32(char_int as u32);
        log::debug!("Output {output:?}");
        log::debug!("Resulting stack is {stack:?}");
        return output
        }

}

fn get_command(artwork: &impl Interpretable, from_index: ChartIndex, from_coord: Coordinate, to_index: ChartIndex, to_coord: Coordinate) -> Option<PietCommand> {
    let from_canvas = artwork.canvas_from_index(from_index)?;
    let from_codel = *from_canvas.get_codel(from_coord);
    let to_codel = *artwork.canvas_from_index(to_index)?.get_codel(to_coord);
    if !from_codel.is_any_colour() || !to_codel.is_any_colour() {return None}
    let block_size = from_canvas.get_block(from_coord)?.size();
    let hue_diff =  from_codel.hue_difference(to_codel);
    let lightness_diff = from_codel.light_difference(to_codel);
    match (hue_diff, lightness_diff) {
        (Some(0),Some(0)) => None,
        (Some(0),Some(1)) => Some(PietCommand::Push(block_size)),
        (Some(0),Some(2)) => Some(PietCommand::Pop),
        (Some(1),Some(0)) => Some(PietCommand::Add),
        (Some(1),Some(1)) => Some(PietCommand::Subtract),
        (Some(1),Some(2)) => Some(PietCommand::Multiply),
        (Some(2),Some(0)) => Some(PietCommand::Divide),
        (Some(2),Some(1)) => Some(PietCommand::Modulo),
        (Some(2),Some(2)) => Some(PietCommand::Not),
        (Some(3),Some(0)) => Some(PietCommand::Greater),
        (Some(3),Some(1)) => Some(PietCommand::Pointer),
        (Some(3),Some(2)) => Some(PietCommand::Switch),
        (Some(4),Some(0)) => Some(PietCommand::Duplicate),
        (Some(4),Some(1)) => Some(PietCommand::Roll),
        (Some(4),Some(2)) => Some(PietCommand::InputNum),
        (Some(5),Some(0)) => Some(PietCommand::InputChar),
        (Some(5),Some(1)) => Some(PietCommand::OutputNum),
        (Some(5),Some(2)) => Some(PietCommand::OutputChar),
        _ => None
    }
}
