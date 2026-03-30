extern crate image;

use crate::str_palette;

#[derive(Debug, Clone, Copy)]
pub struct Colour {
    x: usize,
    y: usize,
    hue: u8,
    light: u8,
    block_size: Option<i64>,
    name: &'static str 
}

impl Colour {
    fn x(&self) -> usize {
        self.x
    }
    fn y(&self) -> usize {
        self.y
    }
    pub fn light(&self) -> u8 {
        self.light
    }
    pub fn hue(&self) -> u8 {
        self.hue
    }
    pub fn block_size(&self) -> Option<i64> {
        self.block_size
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
    White {
        x: usize,
        y: usize,
    },
    Black {
        x: usize,
        y: usize,
    }
}

impl Codel {  
    pub fn new(x: usize, y: usize, colour_name: &str) -> Codel {
        match colour_name {
            str_palette::WHITE => Codel::White {x: x, y: y},
            str_palette::BLACK => Codel::Black {x: x, y: y},

            str_palette::LR => Codel::Colour(Colour {x: x, y: y, hue: 0, light: 0, block_size: None, name: str_palette::LR}),
            str_palette::R => Codel::Colour(Colour {x: x, y: y, hue: 0, light: 1, block_size: None, name: str_palette::R}),
            str_palette::DR => Codel::Colour(Colour {x: x, y: y, hue: 0, light: 2, block_size: None, name: str_palette::DR}),

            str_palette::LY => Codel::Colour(Colour {x: x, y: y, hue: 1, light: 0, block_size: None, name: str_palette::LY}),
            str_palette::Y => Codel::Colour(Colour {x: x, y: y, hue: 1, light: 1, block_size: None, name: str_palette::Y}),
            str_palette::DY => Codel::Colour(Colour {x: x, y: y, hue: 1, light: 2, block_size: None, name: str_palette::DY}),

            str_palette::LG => Codel::Colour(Colour {x: x, y: y, hue: 2, light: 0, block_size: None, name: str_palette::LG}),
            str_palette::G => Codel::Colour(Colour {x: x, y: y, hue: 2, light: 1, block_size: None, name: str_palette::G}),
            str_palette::DG => Codel::Colour(Colour {x: x, y: y, hue: 2, light: 2, block_size: None, name: str_palette::DG}),

            str_palette::LC => Codel::Colour(Colour {x: x, y: y, hue: 3, light: 0, block_size: None, name: str_palette::LC}),
            str_palette::C => Codel::Colour(Colour {x: x, y: y, hue: 3, light: 1, block_size: None, name: str_palette::C}),
            str_palette::DC => Codel::Colour(Colour {x: x, y: y, hue: 3, light: 2, block_size: None, name: str_palette::DC}),

            str_palette::LB => Codel::Colour(Colour {x: x, y: y, hue: 4, light: 0, block_size: None, name: str_palette::LB}),
            str_palette::B => Codel::Colour(Colour {x: x, y: y, hue: 4, light: 1, block_size: None, name: str_palette::B}),
            str_palette::DB => Codel::Colour(Colour {x: x, y: y, hue: 4, light: 2, block_size: None, name: str_palette::DB}),

            str_palette::LM => Codel::Colour(Colour {x: x, y: y, hue: 5, light: 0, block_size: None, name: str_palette::LM}),
            str_palette::M => Codel::Colour(Colour {x: x, y: y, hue: 5, light: 1, block_size: None, name: str_palette::M}),
            str_palette::DM => Codel::Colour(Colour {x: x, y: y, hue: 5, light: 2, block_size: None, name: str_palette::DM}),

            str => panic!("That's not an acceptable colour name: {}. Try again.", str)
        }
    }

    pub fn x(&self) -> usize {
        match self {
            Codel::White {x, y:_} => *x,
            Codel::Black {x, y:_} => *x,
            Codel::Colour(colour) => colour.x()
        }
    }

    pub fn y(&self) -> usize {
        match self {
            Codel::White {x:_, y} => *y,
            Codel::Black { x:_, y } => *y,
            Codel::Colour(colour) => colour.y()
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Codel::White {x:_, y:_} => str_palette::WHITE,
            Codel::Black {x:_, y:_} => str_palette::BLACK,
            Codel::Colour(colour) => colour.name()
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
            Codel::Black { x:_, y:_ } if colour_name == str_palette::BLACK => true,
            Codel::White { x:_, y:_ } if colour_name == str_palette::WHITE => true,
            Codel::Colour(colour) if colour_name == colour.name() => true,
            _ => false
        }
    }

}



pub mod canvas_utils {
    use crate::Codel;
    use image::GenericImageView;
    use crate::rgba_palette;
    use crate::str_palette;
    pub type CanvasError = Box<dyn std::error::Error + Send + Sync + 'static>;
    pub type Canvas = Vec<Vec<Codel>>;

    mod block_size_utils {
        use crate::canvas::canvas_utils::Canvas;

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

    fn codel_from_rgba_and_pos(colour: image::Rgba<u8>, x: usize, y: usize) -> Codel {
        match colour {
            rgba_palette::BLACK => Codel::new(x, y, str_palette::BLACK),
            rgba_palette::WHITE => Codel::new(x, y, str_palette::WHITE),

            rgba_palette::LIGHT_RED => Codel::new(x, y, str_palette::LR),
            rgba_palette::RED => Codel::new(x, y, str_palette::R),
            rgba_palette::DARK_RED => Codel::new(x, y, str_palette::DR),

            rgba_palette::LIGHT_YELLOW => Codel::new(x, y, str_palette::LY),
            rgba_palette::YELLOW => Codel::new(x, y, str_palette::Y),
            rgba_palette::DARK_YELLOW => Codel::new(x, y, str_palette::DY),

            rgba_palette::LIGHT_GREEN => Codel::new(x, y, str_palette::LG),
            rgba_palette::GREEN => Codel::new(x, y, str_palette::G),
            rgba_palette::DARK_GREEN => Codel::new(x, y, str_palette::DG),

            rgba_palette::LIGHT_CYAN => Codel::new(x, y, str_palette::LC),
            rgba_palette::CYAN => Codel::new(x, y, str_palette::C),
            rgba_palette::DARK_CYAN => Codel::new(x, y, str_palette::DC),

            rgba_palette::LIGHT_BLUE => Codel::new(x, y, str_palette::LB),
            rgba_palette::BLUE => Codel::new(x, y, str_palette::B),
            rgba_palette::DARK_BLUE => Codel::new(x, y, str_palette::DB),

            rgba_palette::LIGHT_MAGENTA => Codel::new(x, y, str_palette::LM),
            rgba_palette::MAGENTA => Codel::new(x, y, str_palette::M),
            rgba_palette::DARK_MAGENTA => Codel::new(x, y, str_palette::DM),

            c => {println!("An unrecognised colour was detected: {:?}", c); Codel::new(x, y, str_palette::WHITE)}
        }
    }

    pub fn create_canvas(
        img_path: &'static str, 
        codel_size: u32
    ) -> Result<Canvas, CanvasError> {
        let image = image::open(img_path)?;

        let (unsc_width, unsc_height) = image.dimensions();
        if (!unsc_width.is_multiple_of(codel_size)) || (!unsc_height.is_multiple_of(codel_size)) {
            return Err("Codel size mismatch!".into())
        }
        let (width, height) = (unsc_width/codel_size, unsc_height/codel_size);

        let mut canvas: Canvas = Vec::new();
        for i in 0..width {
            let mut row: Vec<Codel> = Vec::new();
            for j in 0..height {
                let codel = codel_from_rgba_and_pos(
                    image.get_pixel(i*codel_size, j*codel_size), 
                    i as usize, 
                    j as usize);
                row.push(codel);
            }
            canvas.push(row);
        }

        block_size_utils::set_block_sizes(&mut canvas);

        Ok(canvas)
    }
}