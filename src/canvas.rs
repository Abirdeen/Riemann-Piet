extern crate image;
extern crate itertools;

use crate::itertools::Itertools;
use image::{DynamicImage, GenericImageView};
use crate::palette;
use crate::interpreter::{DP, CC};

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
    pub fn name(&self) -> &'static str {
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

    fn from_rgba(colour: image::Rgba<u8>) -> Codel {
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

            c => {log::warn!("An unrecognised colour was detected: {:?}. These colours are treated as black.", c); Codel::new(palette::text::BLACK)}
        }
    }

    pub fn colour_name(&self) -> &'static str {
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
    pub fn is_any_colour(&self) -> bool {
        match self {
            Codel::Colour(_) => true,
            _ => false
        }
    }
    pub fn is_black(&self) -> bool {
        match self {
            Codel::Black {..} => true,
            _ => false
        }
    }

    pub fn hue_difference(&self, codel: Codel) -> Option<u8> {
        match (self.hue(), codel.hue()) {
            (Some(hue1), Some(hue2)) => {
                Some((hue1 as i16 - hue2 as i16).rem_euclid(6) as u8)},
            _ => None
        }
    }
    pub fn light_difference(&self, codel: Codel) -> Option<u8> {
        match (self.light(), codel.light()) {
            (Some(light1), Some(light2)) => Some((light2 as i16 - light1 as i16).rem_euclid(3) as u8),
            _ => None
        }
    }

}

pub type BlockIndex = usize;
pub type BlockSize = usize;

#[derive(Debug, Clone)]
pub struct CodelBlock {
    index: BlockIndex,
    northmost_west_coord: Coordinate,
    northmost_east_coord: Coordinate,
    eastmost_north_coord: Coordinate,
    eastmost_south_coord: Coordinate,
    southmost_east_coord: Coordinate,
    southmost_west_coord: Coordinate,
    westmost_south_coord: Coordinate,
    westmost_north_coord: Coordinate,
    block_size: BlockSize
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

    pub fn get_coord(&self, dp: DP, cc: CC) -> Coordinate {
        match (dp, cc) {
            (DP::North, CC::Left) => {self.northmost_west_coord},
            (DP::North, CC::Right) => {self.northmost_east_coord},
            (DP::East, CC::Left) => {self.eastmost_north_coord},
            (DP::East, CC::Right) => {self.eastmost_south_coord},
            (DP::South, CC::Left) => {self.southmost_east_coord},
            (DP::South, CC::Right) => {self.southmost_west_coord},
            (DP::West, CC::Left) => {self.westmost_south_coord},
            (DP::West, CC::Right) => {self.westmost_north_coord},

        }
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
    pub fn size(&self) -> BlockSize {
        self.block_size
    }

    fn update_northmost(&mut self, coordinate: Coordinate) {
        let (nwx, nwy) = self.get_coord(DP::North, CC::Left);
        let (nex, _) = self.get_coord(DP::North, CC::Right);
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
        let (enx, eny) = self.get_coord(DP::East, CC::Left);
        let (_, esy) = self.get_coord(DP::East, CC::Right);
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
        let (swx, swy) = self.get_coord(DP::South, CC::Right);
        let (sex, _) = self.get_coord(DP::South, CC::Left);
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
        let (wnx, wny) = self.get_coord(DP::West, CC::Right);
        let (_, wsy) = self.get_coord(DP::West, CC::Left);
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

    fn make_codels(image: DynamicImage, codel_size: u32) -> Result<Vec<Vec<Codel>>, CanvasError> {
        let (width, height) = Canvas::scaled_dimensions(&image, codel_size)?;
        let mut codel_map: Vec<Vec<Codel>> = Vec::new();
        for i in 0..width {
            let mut row: Vec<Codel> = Vec::new();
            for j in 0..height {
                let codel = Codel::from_rgba(
                    image.get_pixel(i*codel_size, j*codel_size)
                );
            row.push(codel);
            }
        codel_map.push(row);
        }
        return Ok(codel_map)
    }
    fn from_codels(codel_map: Vec<Vec<Codel>>) -> Box<Canvas> {
        let width = codel_map.len();
        let height = codel_map[0].len();
        let codel_iter: itertools::Product<std::ops::Range<usize>, std::ops::Range<usize>> = (0..width).cartesian_product(0..height);
        let blocks = Vec::new();
        Box::new(Canvas { codel_map, blocks, dimensions: (width, height), codel_iter })
    }
    pub fn new(
        img_path: &str, 
        codel_size: u32
    ) -> Result<Canvas, CanvasError> {
        let image = image::open(img_path)?;
        let codel_map = Canvas::make_codels(image, codel_size)?;
        let mut canvas = *Canvas::from_codels(codel_map);
        canvas.create_blocks();
        Ok(canvas)
    }

    pub fn width(&self) -> usize {
        self.dimensions.0
    }
    pub fn height(&self) -> usize {
        self.dimensions.1
    }
    pub fn dimensions(&self) -> (usize, usize) {
        self.dimensions
    }
    pub fn coordinates_iter(&self) -> itertools::Product<std::ops::Range<usize>, std::ops::Range<usize>> {
        self.codel_iter.clone()
    }
    pub fn blocks_list(&self) -> &Vec<CodelBlock> {
        &self.blocks
    }

    pub fn get_codel(&mut self, coordinate: Coordinate) -> &mut Codel {
        let (x, y) = coordinate;
        &mut self.codel_map[x][y]
    }
    pub fn is_colour(&mut self, coordinate: Coordinate, colour: &str) -> bool {
        self.get_codel(coordinate).is_colour(colour)
    }

    pub fn get_block_from_coord(&mut self, coordinate: Coordinate) -> Option<&mut CodelBlock> {
        let index = self.get_codel(coordinate).block_index()?;
        return Some(&mut self.blocks[index])
    }
    pub fn get_block_from_index(&mut self, index: BlockIndex) -> Option<&CodelBlock> {
        return self.blocks.get(index)
    }

    pub fn north(&self, (x,y): Coordinate) -> Option<Coordinate> {
        if self.is_northmost((x,y)) {
            return None
        }
        return Some((x, y-1))
    }
    pub fn east(&self, (x,y): Coordinate) -> Option<Coordinate> {
        if self.is_eastmost((x,y)) {
            return None
        }
        return Some((x+1, y))
    }
    pub fn south(&self, (x,y): Coordinate) -> Option<Coordinate> {
        if self.is_southmost((x,y)) {
            return None
        }
        return Some((x, y+1))
    }
    pub fn west(&self, (x,y): Coordinate) -> Option<Coordinate> {
        if self.is_westmost((x,y)) {
            return None
        }
        return Some((x-1, y))
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

    fn add_coord_to_block(&mut self, block: &mut CodelBlock, coordinate: Coordinate) {
        if !self.get_codel(coordinate).no_index() {return}
        block.add_coordinate(coordinate);
        self.get_codel(coordinate).set_block_index(block.index());
    }
    fn add_block(&mut self, block: CodelBlock) {
        self.blocks.push(block);
    }

    fn is_west_of((west_x,_): Coordinate, (east_x,_): Coordinate) -> bool {
        return west_x <= east_x
    }
    fn find_colour_west_edge(
        &mut self, 
        block: &mut CodelBlock, 
        coordinate: Coordinate, 
        colour: &str
    ) -> (Option<Coordinate>, bool) {
        if !self.is_colour(coordinate, colour) {
            return (None, false)
        }
        self.add_coord_to_block(block, coordinate);
        let mut west_coord = coordinate;
        let mut pushed_west = false;

        while !self.is_westmost(west_coord) {
            match self.west(west_coord) {
                Some(coord) => {
                    west_coord = coord;
                    if !self.is_colour(west_coord, colour) {
                        return (Some(self.east(west_coord).expect("go west then east => fine?")), pushed_west)
                    }                    
                    pushed_west = true;
                    self.add_coord_to_block(block, west_coord);
                },
                None => {}
            }
        }
        return (Some(west_coord), pushed_west)
    }
    fn find_colour_east_edge(
        &mut self, 
        block: &mut CodelBlock, 
        coordinate: Coordinate, 
        colour: &str
    ) -> Option<Coordinate> {
        if !self.is_colour(coordinate, colour) {
            return None
        }
        self.add_coord_to_block(block, coordinate);
        let mut east_coord = coordinate;

        while !self.is_eastmost(east_coord) {
            match self.east(east_coord) {
                Some(coord) => {
                    east_coord = coord;
                    if !self.is_colour(east_coord, colour) {
                        return Some(self.west(east_coord).expect("go west then east => fine?"))
                    }
                    self.add_coord_to_block(block, east_coord);
                },
                None => {}
            }

        }
        return Some(east_coord)
    }
    fn find_disjoint_intervals(
        &mut self,
        block: &mut CodelBlock,
        colour: &str,
        east_boundary: Coordinate,
        west_seed: Coordinate
    ) -> (Vec<(Coordinate, Coordinate)>, bool) {
        let mut went_east_of_boundary = false;
        let mut intervals: Vec<(Coordinate, Coordinate)> = Vec::new();
        let mut west_edge = west_seed;
        while Self::is_west_of(west_edge, east_boundary) {
            match self.find_colour_east_edge(block, west_edge, colour) {
                Some(east_edge) => {
                    went_east_of_boundary = !Self::is_west_of(east_edge, east_boundary);
                    intervals.push((west_edge, east_edge));
                    match self.east(east_edge) {
                        Some(coord) => west_edge = coord,
                        None => return (intervals, went_east_of_boundary)
                    };
                },
                None => {
                    if west_edge == east_boundary {
                        return (intervals, went_east_of_boundary)
                    }
                    match self.east(west_edge) {
                        Some(coord) => west_edge = coord,
                        None => return (intervals, went_east_of_boundary)
                    }
                }
            };
        };
        return (intervals, went_east_of_boundary)
    }

    fn build_block(&mut self, block: &mut CodelBlock, seed: Coordinate) {
        let codel = self.get_codel(seed);
        if !codel.is_any_colour() {return};
        let colour = codel.colour_name();
        let mut stack: Vec<(usize, usize, usize, i8)> = Vec::new();
        let (x,y) = seed;

        stack.push((y,x,x,1));
        if !self.is_southmost(seed) {stack.push((y+1,x,x,-1));};
        while stack.len() > 0 {
            let (y, xl, xr, dy) = stack.pop().unwrap();
            let (yb, yf) = ((y as i64 + dy as i64) as usize, (y as i64 - dy as i64) as usize);
            if self.get_codel((xl,y)).block_index() == Some(block.index()) {
                continue;
            }
            let mut west_edge = xl;
            match self.find_colour_west_edge(block, (west_edge,y), colour) {
                (Some((new_west, _)), true) => {
                    west_edge = new_west;
                    if (dy == 1 && !self.is_southmost((xl,y))) || (dy==-1 && !self.is_northmost((xl,y))) {
                        stack.push((yb, west_edge, xl-1, -dy));
                    }
                },
                _ => {}
            };
            let (intervals, pushed_east) = self.find_disjoint_intervals(block, colour, (xr,y), (west_edge, y));

            match (pushed_east, intervals.last()) {
                (true, Some(&(_, (new_east, _)))) => {
                    match self.east((xr,y)) {
                        Some((new_west,_)) => {
                            if (dy == 1 && !self.is_southmost((xl,y))) || (dy==-1 && !self.is_northmost((xl,y))) {
                                stack.push((yb, new_west, new_east, -dy));
                            }
                        }
                        None => {}
                    }
                },
                _ => {}
            }
            if (dy == 1 && !self.is_northmost((xl,y))) || (dy==-1 && !self.is_southmost((xl,y))) {
                let new_spans = &mut intervals.iter().map(|((xw,_),(xe,_))| (yf,*xw,*xe,dy)).collect_vec();
                stack.append(new_spans);
            }
        }                
    }
    pub fn create_blocks(&mut self) {
        let mut index = 0;
        for coordinate in self.coordinates_iter().clone() {
            if self.get_codel(coordinate).no_index() {
                let mut block = CodelBlock::new(index, coordinate);
                self.build_block(&mut block, coordinate);
                self.add_block(block);
                index += 1;

            }
        }
    }

}