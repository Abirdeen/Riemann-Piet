use crate::{canvas::{Canvas, CanvasError, Coordinate}, interpreter::{DP, Interpreter}};

pub type ChartIndex = usize;
type ReversalBool = bool;
type ChartTransition<'a> = &'a dyn Fn(&mut Interpreter, ChartIndex) -> Option<(ChartIndex, ReversalBool)>;

pub struct Chart {
    canvas: Canvas,
    index: ChartIndex
}

impl Chart {

    pub fn new(img_path: &str, codel_size: u32, index: ChartIndex) -> Result<Chart, CanvasError> {
        let canvas = Canvas::new(img_path, codel_size)?;
        Ok(Chart {canvas, index})
    }

    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }
    pub fn dimensions(&self) -> (usize, usize) {
        self.canvas.dimensions()
    }
}

pub struct Atlas<'a> {
    charts: Vec<Chart>,
    transition_map: ChartTransition<'a>
}

impl Atlas<'_> {
    pub fn transition_map(&'_ self) -> &ChartTransition<'_> {
        &self.transition_map
    }
    pub fn chart(&self, index: ChartIndex) -> Option<&Chart> {
        let charts = &self.charts;
        if index >= charts.len() {
            return None
        };
        return Some(&charts[index])
    }

    pub fn transition_coord(&self, interpreter: &mut Interpreter, from_index: ChartIndex, to_index: ChartIndex, reverse: ReversalBool) -> Option<Coordinate> {
        let (old_chart_width, old_chart_height) = self.chart(from_index)?.dimensions();
        let (new_chart_width, new_chart_height) = self.chart(to_index)?.dimensions();
        let (mut x, mut y) = interpreter.current_coordinate();
        match interpreter.dp() {
            DP::North|DP::South => {
                if !old_chart_width == new_chart_width {
                    return None
                };
                y = old_chart_height - y - 1;
                if reverse {
                    x = old_chart_height - x - 1;
                }
            },
            DP::West|DP::East => {
                if !old_chart_height == new_chart_height {
                    return None
                };
                x = old_chart_height - x - 1;
                if reverse {
                    y = old_chart_height - y - 1;
                }
            }
        };
        return Some((x,y))
    }
}

impl<'a> Atlas<'a> {

    fn no_transitions(_: &mut Interpreter, _: ChartIndex) -> Option<(ChartIndex, ReversalBool)> {
        return None
    }
    fn torus(_: &mut Interpreter, index: ChartIndex) -> Option<(ChartIndex, ReversalBool)> {
        if index > 0 {
            return None
        };
        return Some((0, false))
    }
    fn klein_bottle(interpreter: &mut Interpreter, index: ChartIndex) -> Option<(ChartIndex, ReversalBool)> {
        if index > 0 {
            return None
        };
        match interpreter.dp() {
            DP::North|DP::South => return Some((0, true)),
            DP::West|DP::East => return Some((0,false))
        }
    }
    fn projective_plane(_: &mut Interpreter, index: ChartIndex) -> Option<(ChartIndex, ReversalBool)> {
        if index > 0 {
            return None
        };
        return Some((0,true))
    }

    pub fn new_canvas(
        img_path: &str, 
        codel_size: u32
    ) -> Result<Atlas<'a>, CanvasError> {
        let chart = Chart::new(img_path, codel_size, 0)?;
        let charts = Vec::from([chart]);
        Ok(Atlas {charts, transition_map: &Atlas::no_transitions})
    }
    pub fn new_torus(
        img_path: &str, 
        codel_size: u32
    ) -> Result<Atlas<'a>, CanvasError> {
        let chart = Chart::new(img_path, codel_size, 0)?;
        let charts = Vec::from([chart]);
        Ok(Atlas {charts, transition_map: &Atlas::torus})
    }
    pub fn new_klein_bottle(
        img_path: &str, 
        codel_size: u32
    ) -> Result<Atlas<'a>, CanvasError> {
        let chart = Chart::new(img_path, codel_size, 0)?;
        let charts = Vec::from([chart]);
        Ok(Atlas {charts, transition_map: &Atlas::klein_bottle})
    }
    pub fn new_projective_plane(
        img_path: &str, 
        codel_size: u32
    ) -> Result<Atlas<'a>, CanvasError> {
        let chart = Chart::new(img_path, codel_size, 0)?;
        let charts = Vec::from([chart]);
        Ok(Atlas {charts, transition_map: &Atlas::projective_plane})
    }
}