use itertools::Itertools;

use crate::canvas::{BlockIndex, CodelBlock};
use crate::interpreter::{CC, CodeState, DP, Artwork, Interpreter, PietCommand, PointerAspect};
use crate::surface::ChartIndex;

#[derive(Clone, Copy)]
struct BlockTransition {
    new_chart_index: ChartIndex,
    new_block_index: BlockIndex,
    command: Option<PietCommand>,
    codel_chooser: CC,
    direction_pointer: DP
}

#[derive(Clone, Copy)]
enum Operation {
    Continue(BlockTransition),
    ModifyPointer,
    Terminate
}

struct LexedBlock {
    index: BlockIndex,
    transitions: [[Operation;4];2],
}

impl LexedBlock {
    fn new(artwork: &impl Artwork, chart_index: ChartIndex, block: &CodelBlock) -> LexedBlock {
        let directions = [DP::North, DP::East, DP::South, DP::West].iter();
        let cc_options = [CC::Left, CC::Right].iter();
        let mut transitions_vec: Vec<Vec<Operation>> = Vec::new();

        for (dp, cc) in directions.cartesian_product(cc_options) {
            let exit_coord = block.get_coord(*dp, *cc);
            let mut interpreter = Interpreter::from_data(*dp, *cc, exit_coord, chart_index, false);
            transitions_vec[*dp as usize][*cc as usize] = LexedBlock::get_block_transition(artwork, &mut interpreter);
        };
        let transitions: [[Operation;4];2] = [
            [
            transitions_vec[DP::North as usize][CC::Left as usize],
            transitions_vec[DP::East as usize][CC::Left as usize],
            transitions_vec[DP::South as usize][CC::Left as usize],
            transitions_vec[DP::West as usize][CC::Left as usize]
            ], 
            [
            transitions_vec[DP::North as usize][CC::Right as usize],
            transitions_vec[DP::East as usize][CC::Right as usize],
            transitions_vec[DP::South as usize][CC::Right as usize],
            transitions_vec[DP::West as usize][CC::Right as usize]
            ]
            ];
        return LexedBlock { index: block.index(), transitions: transitions }
    }

    fn get_block_transition(
        artwork: &impl Artwork,
        interpreter: &mut Interpreter
    ) -> Operation {
        let aspect = PointerAspect::CC;
        loop {
            match interpreter.get_next_state(artwork, aspect) {
                CodeState::Continue(command) => {
                    let current_canvas = artwork.current_canvas(interpreter);
                    match current_canvas.get_block(interpreter.current_coordinate()) {
                        Some(block) => {
                            let transition = BlockTransition { 
                                new_chart_index: interpreter.current_chart_index(), 
                                new_block_index: block.index(), 
                                command, 
                                codel_chooser: interpreter.cc(), 
                                direction_pointer: interpreter.dp()
                            };
                            return Operation::Continue(transition)
                        },
                        None => {continue}
                    }
                },
                CodeState::ModifyPointer(_) => return Operation::ModifyPointer,
                CodeState::Terminate|CodeState::Error => return Operation::Terminate
            }            
        }
    }

    fn eliminate_dead_ends(&self) -> LexedBlock {
        let directions = [DP::North, DP::East, DP::South, DP::West].iter();
        let cc_options = [CC::Left, CC::Right].iter();
        let mut transitions = self.transitions;

        'Outer: for (dp, cc) in directions.cartesian_product(cc_options) {
            let mut shadow_cc = *cc;
            let mut shadow_dp = *dp;
            for _ in 0..4 {
                match transitions[shadow_dp as usize][shadow_cc as usize] {
                    Operation::ModifyPointer => {
                        shadow_cc = shadow_cc.flip()
                    }
                    op => {
                        transitions[*dp as usize][*cc as usize] = op;
                        continue 'Outer
                    }
                }
                match transitions[shadow_dp as usize][shadow_cc as usize] {
                    Operation::ModifyPointer => {
                        shadow_dp = shadow_dp.rotate()
                    }
                    op => {
                        transitions[*dp as usize][*cc as usize] = op;
                        continue 'Outer
                    }
                }
            }
            transitions[*dp as usize][*cc as usize] = Operation::Terminate
        };
        return LexedBlock { index: self.index, transitions }
    }
}

struct LexedChart {
    blocks: Vec<LexedBlock>,
    index: ChartIndex
}

impl LexedChart {
    fn new(blocks: Vec<LexedBlock>, index: ChartIndex) -> LexedChart {
        LexedChart { blocks, index }
    }

    fn eliminate_dead_ends(&self) -> LexedChart {
        let mut blocks: Vec<LexedBlock> = Vec::new();
        for block in &self.blocks {
            blocks.push(block.eliminate_dead_ends());
        };
        return LexedChart { blocks, index: self.index }
    }
}

pub struct LexedArtwork {
    charts: Vec<LexedChart>
}

impl LexedArtwork {
    fn new(artwork: &impl Artwork) -> LexedArtwork {
        let mut charts: Vec<LexedChart> = Vec::new();
        for index in 0..artwork.max_chart_index() {
            let canvas = match artwork.canvas_from_index(index) {
                Some(c) => c,
                None => continue
            };
            let mut blocks: Vec<LexedBlock> = Vec::new();
            for block in canvas.blocks() {
                blocks.push(LexedBlock::new(artwork, index, block));
            }
            let lexed_chart = LexedChart::new(blocks, index);
            charts.push(lexed_chart);
        };
        LexedArtwork { charts }
    }

    fn eliminate_dead_ends(&self) -> LexedArtwork {
        let mut charts: Vec<LexedChart> = Vec::new();
        for chart in &self.charts {
            charts.push(chart.eliminate_dead_ends());
        };
        return LexedArtwork { charts }
    }
}

pub struct Lexer {}

impl Lexer {
    fn first_pass(artwork: &impl Artwork) -> LexedArtwork {
        return LexedArtwork::new(artwork)
    }

    fn second_pass(artwork: LexedArtwork) -> LexedArtwork {
        return artwork.eliminate_dead_ends()
    }

    pub fn lex(artwork: &impl Artwork) -> LexedArtwork {
        return Lexer::second_pass(Lexer::first_pass(artwork))
    }
}