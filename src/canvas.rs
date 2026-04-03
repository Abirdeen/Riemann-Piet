extern crate image;
extern crate itertools;

use crate::itertools::Itertools;
use crate::palette;

pub type CanvasError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Coordinate = (usize, usize);

#[derive(Debug, Clone, Copy)]
pub struct Colour {
    hue: u8,
    light: u8,
    block_index: Option<usize>,
    name: &'static str 
}

impl Colour {
    fn hue(&self) -> u8 {
        self.hue
    }
    fn light(&self) -> u8 {
        self.light
    }
    fn block_index(&self) -> Option<usize> {
        self.block_index
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn set_block_index(&mut self, new_index: usize) {
        self.block_index = Some(new_index);
        return
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Codel {
    Colour(Colour),
    White,
    Black
}

impl Codel {  
    pub fn new(colour_name: &str) -> Codel {
        match colour_name {
            palette::text::WHITE => Codel::White,
            palette::text::BLACK => Codel::Black,

            palette::text::LR => Codel::Colour(Colour {hue: 5, light: 0, block_index: None, name: palette::text::LR}),
            palette::text::R => Codel::Colour(Colour {hue: 5, light: 1, block_index: None, name: palette::text::R}),
            palette::text::DR => Codel::Colour(Colour {hue: 5, light: 2, block_index: None, name: palette::text::DR}),

            palette::text::LY => Codel::Colour(Colour {hue: 4, light: 0, block_index: None, name: palette::text::LY}),
            palette::text::Y => Codel::Colour(Colour {hue: 4, light: 1, block_index: None, name: palette::text::Y}),
            palette::text::DY => Codel::Colour(Colour {hue: 4, light: 2, block_index: None, name: palette::text::DY}),

            palette::text::LG => Codel::Colour(Colour {hue: 3, light: 0, block_index: None, name: palette::text::LG}),
            palette::text::G => Codel::Colour(Colour {hue: 3, light: 1, block_index: None, name: palette::text::G}),
            palette::text::DG => Codel::Colour(Colour {hue: 3, light: 2, block_index: None, name: palette::text::DG}),

            palette::text::LC => Codel::Colour(Colour {hue: 2, light: 0, block_index: None, name: palette::text::LC}),
            palette::text::C => Codel::Colour(Colour {hue: 2, light: 1, block_index: None, name: palette::text::C}),
            palette::text::DC => Codel::Colour(Colour {hue: 2, light: 2, block_index: None, name: palette::text::DC}),

            palette::text::LB => Codel::Colour(Colour {hue: 1, light: 0, block_index: None, name: palette::text::LB}),
            palette::text::B => Codel::Colour(Colour {hue: 1, light: 1, block_index: None, name: palette::text::B}),
            palette::text::DB => Codel::Colour(Colour {hue: 1, light: 2, block_index: None, name: palette::text::DB}),

            palette::text::LM => Codel::Colour(Colour {hue: 0, light: 0, block_index: None, name: palette::text::LM}),
            palette::text::M => Codel::Colour(Colour {hue: 0, light: 1, block_index: None, name: palette::text::M}),
            palette::text::DM => Codel::Colour(Colour {hue: 0, light: 2, block_index: None, name: palette::text::DM}),

            str => panic!("That's not an acceptable colour name: {}. Try again.", str)
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Codel::White {..} => palette::text::WHITE,
            Codel::Black {..} => palette::text::BLACK,
            Codel::Colour(colour) => colour.name()
        }
    }
    pub fn hue(&self) -> Option<u8> {
        match self {
            Codel::Colour(colour) => Some(colour.hue()),
            _ => None
        }
    }
    pub fn light(&self) -> Option<u8> {
        match self {
            Codel::Colour(colour) => Some(colour.light()),
            _ => None
        }
    }
    pub fn block_index(&self) -> Option<usize> {
        match self {
            Codel::Colour(colour) => colour.block_index(),
            _ => None
        }
    }

    pub fn set_block_index(&mut self, new_index: usize) {
        match self {
            Codel::Colour(colour) => colour.set_block_index(new_index),
            _ => ()
        }
    }
    pub fn no_index(&self) -> bool {
        match self {
            Codel::Colour(colour) if colour.block_index() == None => true,
            _ => false
        }
    }

    pub fn is_colour(&self, colour_name: &str) -> bool {
        match self {
            Codel::Black {..} if colour_name == palette::text::BLACK => true,
            Codel::White {..} if colour_name == palette::text::WHITE => true,
            Codel::Colour(colour) if colour_name == colour.name() => true,
            _ => false
        }
    }
#[derive(Debug, Clone)]
pub struct CodelBlock {
    index: usize,
    northmost_west_coord: Coordinate,
    northmost_east_coord: Coordinate,
    eastmost_north_coord: Coordinate,
    eastmost_south_coord: Coordinate,
    southmost_east_coord: Coordinate,
    southmost_west_coord: Coordinate,
    westmost_south_coord: Coordinate,
    westmost_north_coord: Coordinate,
    block_size: u32
}

impl CodelBlock {
    pub fn new(index: usize, coordinate: Coordinate) -> CodelBlock {
        CodelBlock { index, 
            northmost_west_coord: coordinate, 
            northmost_east_coord: coordinate, 
            eastmost_north_coord: coordinate, 
            eastmost_south_coord: coordinate, 
            southmost_east_coord: coordinate, 
            southmost_west_coord: coordinate, 
            westmost_south_coord: coordinate, 
            westmost_north_coord: coordinate, 
            block_size: 0 }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn northmost_west(&self) -> Coordinate {
        self.northmost_west_coord
    }
    pub fn northmost_east(&self) -> Coordinate {
        self.northmost_east_coord
    }
    pub fn eastmost_north(&self) -> Coordinate {
        self.eastmost_north_coord
    }
    pub fn eastmost_south(&self) -> Coordinate {
        self.eastmost_south_coord
    }
    pub fn southmost_east(&self) -> Coordinate {
        self.southmost_east_coord
    }
    pub fn southmost_west(&self) -> Coordinate {
        self.southmost_west_coord
    }
    pub fn westmost_south(&self) -> Coordinate {
        self.westmost_south_coord
    }
    pub fn westmost_north(&self) -> Coordinate {
        self.westmost_north_coord
    }
    pub fn size(&self) -> u32 {
        self.block_size
    }

    fn update_northmost(&mut self, coordinate: Coordinate) {
        let (nwx, nwy) = self.northmost_west();
        let (nex, _) = self.northmost_east();
        match coordinate {
            (_,y) if y > nwy => (),
            (_,y) if y < nwy => {
                self.northmost_west_coord = coordinate;
                self.northmost_east_coord = coordinate;
            },
            (x,_) if x < nwx => self.northmost_west_coord = coordinate,
            (x,_) if x > nex => self.northmost_east_coord = coordinate,
            _ => ()
        }
    }
    fn update_eastmost(&mut self, coordinate: Coordinate) {
        let (enx, eny) = self.eastmost_north();
        let (_, esy) = self.eastmost_south();
        match coordinate {
            (x,_) if x < enx => (),
            (x,_) if x > enx => {
                self.eastmost_north_coord = coordinate;
                self.eastmost_south_coord = coordinate;
            },
            (_,y) if y < eny => self.eastmost_north_coord = coordinate,
            (_,y) if y > esy => self.eastmost_south_coord = coordinate,
            _ => ()
        }
    }
    fn update_southmost(&mut self, coordinate: Coordinate) {
        let (swx, swy) = self.southmost_west();
        let (sex, _) = self.southmost_east();
        match coordinate {
            (_,y) if y < swy => (),
            (_,y) if y > swy => {
                self.southmost_west_coord = coordinate;
                self.southmost_east_coord = coordinate;
            },
            (x,_) if x < swx => self.southmost_west_coord = coordinate,
            (x,_) if x > sex => self.southmost_east_coord = coordinate,
            _ => ()
        }
    }
    fn update_westmost(&mut self, coordinate: Coordinate) {
        let (wnx, wny) = self.westmost_north();
        let (_, wsy) = self.westmost_south();
        match coordinate {
            (x,_) if x > wnx => (),
            (x,_) if x < wnx => {
                self.westmost_north_coord = coordinate;
                self.westmost_south_coord = coordinate;
            },
            (_,y) if y < wny => self.westmost_north_coord = coordinate,
            (_,y) if y > wsy => self.westmost_south_coord = coordinate,
            _ => ()
        }
    }

    pub fn add_coordinate(&mut self, coordinate: Coordinate) {
        self.block_size += 1;
        self.update_northmost(coordinate);
        self.update_eastmost(coordinate);
        self.update_southmost(coordinate);
        self.update_westmost(coordinate);
    }

}

#[derive(Debug, Clone)]
pub struct Canvas {
    codel_map: Vec<Vec<Codel>>,
    blocks: Vec<CodelBlock>,
    dimensions: (usize, usize),
    codel_iter: itertools::Product<std::ops::Range<usize>, std::ops::Range<usize>>
}

impl Canvas {
    pub fn new(codel_map: Vec<Vec<Codel>>) -> Box<Canvas> {
        let width = codel_map.len();
        let height = codel_map[0].len();
        let codel_iter: itertools::Product<std::ops::Range<usize>, std::ops::Range<usize>> = (0..width).cartesian_product(0..height);
        let blocks = Vec::new();
        Box::new(Canvas { codel_map, blocks, dimensions: (width, height), codel_iter })
    }

    pub fn dimensions(&self) -> (usize, usize) {
        self.dimensions
    }
    pub fn coordinates_iter(&self) -> itertools::Product<std::ops::Range<usize>, std::ops::Range<usize>> {
        self.codel_iter.clone()
    }

    pub fn get_codel(&mut self, coordinate: Coordinate) -> &mut Codel {
        let (x, y) = coordinate;
        &mut self.codel_map[x][y]
    }
    pub fn get_block_from_coord(&mut self, coordinate: Coordinate) -> Option<&mut CodelBlock> {
        let index = self.get_codel(coordinate).block_index()?;
        return Some(&mut self.blocks[index])
    }

    pub fn north(&self, (x,y): Coordinate) -> Coordinate {
        return (x, y-1)
    }
    pub fn east(&self, (x,y): Coordinate) -> Coordinate {
        return (x+1, y)
    }
    pub fn south(&self, (x,y): Coordinate) -> Coordinate {
        return (x, y+1)
    }
    pub fn west(&self, (x,y): Coordinate) -> Coordinate {
        return (x-1, y)
    }

    pub fn is_northmost(&self, (_,y): Coordinate) -> bool {
        return y==0
    }
    pub fn is_eastmost(&self, (x,_): Coordinate) -> bool {
        return x==((self.dimensions().0) - 1)
    }
    pub fn is_southmost(&self, (_,y): Coordinate) -> bool {
        return y==((self.dimensions().1) - 1)
    }
    pub fn is_westmost(&self, (x,_): Coordinate) -> bool {
        return x==0
    }

    pub fn add_block(&mut self, block: CodelBlock) {
        self.blocks.push(block);
    }
}

mod canvas_utils {
    use crate::{Canvas, Codel};
    use crate::canvas::CanvasError;
    use crate::palette;
    use image::{DynamicImage, GenericImageView};

    mod block_utils {
        use crate::Canvas;
        use crate::canvas::{CodelBlock, Coordinate};

        fn try_cardinal_fills(
            canvas: &mut Canvas, 
            coordinate: Coordinate, 
            colour_name: &str, 
            block: &mut CodelBlock
        ) {
            if !canvas.is_northmost(coordinate) {
                build_block(canvas, canvas.north(coordinate), colour_name, block);
            };
            if !canvas.is_eastmost(coordinate) {
                build_block(canvas, canvas.east(coordinate), colour_name, block);
            };
            if !canvas.is_southmost(coordinate) {
                build_block(canvas, canvas.south(coordinate), colour_name, block);
            };
            if !canvas.is_westmost(coordinate) {
                build_block(canvas, canvas.west(coordinate), colour_name, block);
            };
        }

        fn build_block(
            canvas: &mut Canvas, 
            coordinate: Coordinate, 
            colour_name: &str, 
            block: &mut CodelBlock
        ) {
            let codel = canvas.get_codel(coordinate);
            if !codel.no_index() {
                return ()
            }
            if !codel.is_colour(colour_name) {
                return ()
            }

            block.add_coordinate(coordinate);
            codel.set_block_index(block.index());

            try_cardinal_fills(canvas, coordinate, colour_name, block)
        }

        // This algorithm is horribly unoptimised; a better version is Paul Heckbert's span fill. Not a huge worry, as this is a compile-time problem.
        // Note: this also doesn't cover blocks that span canvases! Implicitly imagines a thin white frame around each canvas.
        pub fn set_block_sizes(canvas: &mut Canvas) {
            let mut index = 0;
            for coordinate in canvas.coordinates_iter() {
                if canvas.get_codel(coordinate).no_index() {
                    let mut block = CodelBlock::new(index, coordinate);
                    let colour_name = canvas.get_codel(coordinate).name();
                    build_block(canvas, coordinate, colour_name, &mut block);
                    canvas.add_block(block);
                    index += 1;
                }
            }
        }

    }

    fn codel_from_rgba_and_coord(colour: image::Rgba<u8>) -> Codel {
        match colour {
            palette::rgba::BLACK => Codel::new(palette::text::BLACK),
            palette::rgba::WHITE => Codel::new(palette::text::WHITE),

            palette::rgba::LIGHT_RED => Codel::new(palette::text::LR),
            palette::rgba::RED => Codel::new(palette::text::R),
            palette::rgba::DARK_RED => Codel::new(palette::text::DR),

            palette::rgba::LIGHT_YELLOW => Codel::new(palette::text::LY),
            palette::rgba::YELLOW => Codel::new(palette::text::Y),
            palette::rgba::DARK_YELLOW => Codel::new(palette::text::DY),

            palette::rgba::LIGHT_GREEN => Codel::new(palette::text::LG),
            palette::rgba::GREEN => Codel::new(palette::text::G),
            palette::rgba::DARK_GREEN => Codel::new(palette::text::DG),

            palette::rgba::LIGHT_CYAN => Codel::new(palette::text::LC),
            palette::rgba::CYAN => Codel::new(palette::text::C),
            palette::rgba::DARK_CYAN => Codel::new(palette::text::DC),

            palette::rgba::LIGHT_BLUE => Codel::new(palette::text::LB),
            palette::rgba::BLUE => Codel::new(palette::text::B),
            palette::rgba::DARK_BLUE => Codel::new(palette::text::DB),

            palette::rgba::LIGHT_MAGENTA => Codel::new(palette::text::LM),
            palette::rgba::MAGENTA => Codel::new(palette::text::M),
            palette::rgba::DARK_MAGENTA => Codel::new(palette::text::DM),

            c => {println!("An unrecognised colour was detected: {:?}. These colours are treated as white.", c); Codel::new(palette::text::WHITE)}
        }
    }

    fn scaled_dimensions(
        image: &DynamicImage, 
        codel_size: u32
    ) -> Result<(u32, u32), CanvasError> {
        let (unsc_width, unsc_height) = image.dimensions();
        if (!unsc_width.is_multiple_of(codel_size)) || (!unsc_height.is_multiple_of(codel_size)) {
            return Err("Codel size mismatch!".into())
        };
        return Ok((unsc_width/codel_size, unsc_height/codel_size))
    }

    pub fn create_canvas(
        img_path: &str, 
        codel_size: u32
    ) -> Result<Canvas, CanvasError> {
        let image = image::open(img_path)?;
        let (width, height) = scaled_dimensions(&image, codel_size)?;

        let mut codel_map: Vec<Vec<Codel>> = Vec::new();
        for i in 0..width {
            let mut row: Vec<Codel> = Vec::new();
            for j in 0..height {
                let codel = codel_from_rgba_and_coord(
                    image.get_pixel(i*codel_size, j*codel_size)
                    );
                row.push(codel);
            }
            codel_map.push(row);
        }

        let mut canvas = *Canvas::new(codel_map);

        block_utils::set_block_sizes(&mut canvas);

        Ok(canvas)
    }
}

pub fn create_canvas(
        img_path: &str, 
        codel_size: u32
    ) -> Result<Canvas, CanvasError> {
    canvas_utils::create_canvas(img_path, codel_size)
}