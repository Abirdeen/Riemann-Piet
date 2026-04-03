
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

pub mod text {
    pub const WHITE: &'static str = "White";
    pub const BLACK: &'static str = "Black";

    pub const LR: &'static str = "Light red";
    pub const R: &'static str = "Red";
    pub const DR: &'static str = "Dark red";

    pub const LY: &'static str = "Light yellow";
    pub const Y: &'static str = "Yellow";
    pub const DY: &'static str = "Dark yellow";

    pub const LG: &'static str = "Light green";
    pub const G: &'static str = "Green";
    pub const DG: &'static str = "Dark green";

    pub const LC: &'static str = "Light cyan";
    pub const C: &'static str = "Cyan";
    pub const DC: &'static str = "Dark cyan";

    pub const LB: &'static str = "Light blue";
    pub const B: &'static str = "Blue";
    pub const DB: &'static str = "Dark blue";

    pub const LM: &'static str = "Light magenta";
    pub const M: &'static str = "Magenta";
    pub const DM: &'static str = "Dark magenta";
}