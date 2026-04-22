extern crate image;
extern crate itertools;
extern crate log;
extern crate env_logger;

#[cfg(test)]
mod tests;

mod palette;
mod canvas;
mod surface;
mod interpreter;
mod lexer;

use crate::canvas::Canvas;
use crate::interpreter::Interpreter;
use crate::surface::Atlas;

fn main() {}