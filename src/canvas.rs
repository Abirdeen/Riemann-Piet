extern crate image;
use crate::palette;

use crate::str_palette;

#[derive(Debug, Clone, Copy)]
pub struct Colour {
    hue: u8,
    light: u8,
    block_size: Option<i64>,
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
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn set_block_size(&mut self, new_size: i64) {
        self.block_size = Some(new_size);
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

    pub fn set_block_size(&mut self, new_size: i64) {
        match self {
            Codel::Colour(colour) => colour.set_block_size(new_size),
            _ => ()
        }
    }

    pub fn mark(&mut self) {
        self.set_block_size(-1);
    }

    pub fn is_marked(&self) -> bool {
        match self {
            Codel::Colour(colour) if colour.block_size() == Some(-1) => true,
            _ => false
        }
    }

    pub fn block_size_is_none(&self) -> bool {
        match self {
            Codel::Colour(colour) if colour.block_size() == None => true,
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

}

pub type CanvasError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Canvas = Vec<Vec<Codel>>;

mod canvas_utils {
    use crate::{Canvas, Codel};
    use crate::canvas::CanvasError;
    use crate::palette;
    use image::{DynamicImage, GenericImageView};

    mod block_utils {
        use crate::Canvas;

        fn fill(canvas: &mut Canvas, x: usize, y: usize, colour_name: &str, counted_block: i64) -> i64 {

            let codel = &mut canvas[x][y];
            if codel.is_marked() {
                return counted_block
            }
            if !codel.is_colour(colour_name) {
                return counted_block
            }

            codel.mark();

            let mut new_count = counted_block + 1;
            if x > 0 {new_count = fill(canvas, x-1, y, colour_name, new_count);}
            if y > 0 {new_count = fill(canvas, x, y-1, colour_name, new_count);}
            if x+1 < canvas.len() {new_count = fill(canvas, x+1, y, colour_name, new_count);}
            if y+1 < canvas[0].len() {new_count = fill(canvas, x, y+1, colour_name, new_count);}

            return new_count
        }

        fn get_block_size_and_mark(canvas: &mut Canvas, x: usize, y: usize) -> i64 {
            let colour_name = canvas[x][y].name();
            return fill(canvas, x, y, colour_name, 0)
        }

        fn set_block_size_at_marked(canvas: &mut Canvas, block_size: i64) {
            for x in 0..canvas.len() {
                for y in 0..canvas[x].len() {
                    if canvas[x][y].is_marked() {
                        canvas[x][y].set_block_size(block_size);
                    }
                }
            }
        }

        // This algorithm is horribly unoptimised; a better version is Paul Heckbert's span fill. Not a huge worry, as this is a compile-time problem.
        // Note: this also doesn't cover blocks that span canvases! Implicitly imagines a thin white frame around each canvas.
        pub fn set_block_sizes(canvas: &mut Canvas) {
            for x in 0..canvas.len() {
                for y in 0..canvas[x].len() {
                    if canvas[x][y].block_size_is_none() {
                        let block_size = get_block_size_and_mark(canvas, x, y);
                        set_block_size_at_marked(canvas, block_size);
                    }
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

        let mut canvas: Canvas = Vec::new();
        for i in 0..width {
            let mut row: Vec<Codel> = Vec::new();
            for j in 0..height {
                let codel = codel_from_rgba_and_coord(
                    image.get_pixel(i*codel_size, j*codel_size)
                    );
                row.push(codel);
            }
            canvas.push(row);
        }

        block_size_utils::set_block_sizes(&mut canvas);


        Ok(canvas)
    }
}

pub fn create_canvas(
        img_path: &str, 
        codel_size: u32
    ) -> Result<Canvas, CanvasError> {
    canvas_utils::create_canvas(img_path, codel_size)
}