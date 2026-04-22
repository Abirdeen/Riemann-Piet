use crate::canvas::Canvas;
use crate::palette::PietColour;

const PATH: &'static str = "./src/tests/test_images/";

mod creation {
    use crate::canvas::Canvas;
    use crate::tests::canvas_tests::PATH;

    #[test]
    fn image_loads() {
        let img_path = PATH.to_owned() + "10x10_black.png";
        let codel_size = 10;
        let canvas = Canvas::new(&img_path, codel_size);
        assert!(matches!(canvas, Ok(_)))
    }

    #[test]
    fn error_on_malformed_filepath() {
        let img_path = "not a filepath";
        let error = Canvas::new(img_path, 1);
        assert!(matches!(error, Err(_)))
    }

    #[test]
    fn error_on_wrong_codel_size() {
        let img_path = PATH.to_owned() + "10x10_black.png";
        let error = Canvas::new(&img_path, 3);
        assert!(matches!(error, Err(_)))
    }
}

#[test]
fn uses_codel_size() {
    let img_path = PATH.to_owned() + "10x10_black.png";
    let codel_size_1 = 10;
    let canvas_1x1 = Canvas::new(&img_path, codel_size_1).expect("Failed to load canvas");
    let codel_size_2 = 1;
    let canvas_10x10 = Canvas::new(&img_path, codel_size_2).expect("Failed to load canvas");
    assert_eq!((1,1), canvas_1x1.dimensions());
    assert_eq!((10,10), canvas_10x10.dimensions());
}

#[test]
fn reads_palette() {
    let img_path = PATH.to_owned() + "palette.png";
    let codel_size = 1;
    let canvas = Canvas::new(&img_path, codel_size).expect("Failed to load canvas");
    assert!(canvas.is_colour((0,0), PietColour::LightRed));
    assert!(canvas.is_colour((0,1), PietColour::Red));
    assert!(canvas.is_colour((0,2), PietColour::DarkRed));
    assert!(canvas.is_colour((1,0), PietColour::LightYellow));
    assert!(canvas.is_colour((1,1), PietColour::Yellow));
    assert!(canvas.is_colour((1,2), PietColour::DarkYellow));
    assert!(canvas.is_colour((2,0), PietColour::LightGreen));
    assert!(canvas.is_colour((2,1), PietColour::Green));
    assert!(canvas.is_colour((2,2), PietColour::DarkGreen));
    assert!(canvas.is_colour((3,0), PietColour::LightCyan));
    assert!(canvas.is_colour((3,1), PietColour::Cyan));
    assert!(canvas.is_colour((3,2), PietColour::DarkCyan));
    assert!(canvas.is_colour((4,0), PietColour::LightBlue));
    assert!(canvas.is_colour((4,1), PietColour::Blue));
    assert!(canvas.is_colour((4,2), PietColour::DarkBlue));
    assert!(canvas.is_colour((5,0), PietColour::LightMagenta));
    assert!(canvas.is_colour((5,1), PietColour::Magenta));
    assert!(canvas.is_colour((5,2), PietColour::DarkMagenta));
    assert!(canvas.is_colour((6,0), PietColour::White));
    assert!(canvas.is_colour((6,1), PietColour::Black));
    assert!(!canvas.is_colour((6,2), PietColour::Black));
}

#[test]
fn groks_simple_codel_block() {
    let img_path = PATH.to_owned() + "blocks.png";
    let codel_size = 1;
    let canvas = Canvas::new(&img_path, codel_size).expect("Failed to load canvas");
    let simple_block = canvas.get_block((0,0)).expect("Failed to build block");
    assert_eq!(simple_block.size(), 121);
    assert_eq!(simple_block.index(), 0);
    assert_eq!(
        [simple_block.northmost_west(), simple_block.northmost_east(),
        simple_block.eastmost_north(), simple_block.eastmost_south(),
        simple_block.southmost_east(), simple_block.southmost_west(),
        simple_block.westmost_south(), simple_block.westmost_north()],
        [(0,0), (10,0),
        (10,0), (10,10),
        (10,10), (0,10),
        (0,10), (0,0)]
    );
}

#[test]
fn groks_complex_codel_block() {
    let img_path = PATH.to_owned() + "blocks.png";
    let codel_size = 1;
    let canvas = Canvas::new(&img_path, codel_size).expect("Failed to load canvas");
    let complex_block_1 = canvas.get_block((11,0)).expect("Failed to build block");  
    let complex_block_2 = canvas.get_block((32,0)).expect("Failed to build block");
    assert_eq!(complex_block_1.size(), 133);
    assert_eq!(complex_block_1.index(), 1);
    assert_eq!(
        [complex_block_1.northmost_west(), complex_block_1.northmost_east(),
        complex_block_1.eastmost_north(), complex_block_1.eastmost_south(),
        complex_block_1.southmost_east(), complex_block_1.southmost_west(),
        complex_block_1.westmost_south(), complex_block_1.westmost_north()],
        [(11,0), (31,0),
        (31,0), (31,10),
        (31,10), (11,10),
        (11,10), (11,0)]
    );
    assert_eq!(complex_block_2.size(), 120);
    assert_eq!(complex_block_2.index(), 2);
    assert_eq!(
        [complex_block_2.northmost_west(), complex_block_2.northmost_east(),
        complex_block_2.eastmost_north(), complex_block_2.eastmost_south(),
        complex_block_2.southmost_east(), complex_block_2.southmost_west(),
        complex_block_2.westmost_south(), complex_block_2.westmost_north()],
        [(32,0), (50,0),
        (51,2), (51,2),
        (50,10), (32,10),
        (32,10), (32,0)]
    );
}