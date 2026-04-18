
pub mod rgba {
    use image::Rgba;

    pub const WHITE: Rgba<u8> = Rgba::<u8>([255,255,255,255]);
    pub const BLACK: Rgba<u8> = Rgba::<u8>([0,0,0,255]);

    pub const LIGHT_RED: Rgba<u8> = Rgba::<u8>([255,192,192,255]);
    pub const RED: Rgba<u8> = Rgba::<u8>([255,0,0,255]);
    pub const DARK_RED: Rgba<u8> = Rgba::<u8>([192,0,0,255]);

    pub const LIGHT_YELLOW: Rgba<u8> = Rgba::<u8>([255,255,192,255]);
    pub const YELLOW: Rgba<u8> = Rgba::<u8>([255,255,0,255]);
    pub const DARK_YELLOW: Rgba<u8> = Rgba::<u8>([192,192,0,255]);

    pub const LIGHT_GREEN: Rgba<u8> = Rgba::<u8>([192,255,192,255]);
    pub const GREEN: Rgba<u8> = Rgba::<u8>([0,255,0,255]);
    pub const DARK_GREEN: Rgba<u8> = Rgba::<u8>([0,192,0,255]);

    pub const LIGHT_CYAN: Rgba<u8> = Rgba::<u8>([192,255,255,255]);
    pub const CYAN: Rgba<u8> = Rgba::<u8>([0,255,255,255]);
    pub const DARK_CYAN: Rgba<u8> = Rgba::<u8>([0,192,192,255]);

    pub const LIGHT_BLUE: Rgba<u8> = Rgba::<u8>([192,192,255,255]);
    pub const BLUE: Rgba<u8> = Rgba::<u8>([0,0,255,255]);
    pub const DARK_BLUE: Rgba<u8> = Rgba::<u8>([0,0,192,255]);

    pub const LIGHT_MAGENTA: Rgba<u8> = Rgba::<u8>([255,192,255,255]);
    pub const MAGENTA: Rgba<u8> = Rgba::<u8>([255,0,255,255]);
    pub const DARK_MAGENTA: Rgba<u8> = Rgba::<u8>([192,0,192,255]);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PietColour {
    White,
    Black,

    LightRed,
    Red,
    DarkRed,

    LightYellow,
    Yellow,
    DarkYellow,

    LightGreen,
    Green,
    DarkGreen,

    LightCyan,
    Cyan,
    DarkCyan,

    LightBlue,
    Blue,
    DarkBlue,

    LightMagenta,
    Magenta,
    DarkMagenta
}
