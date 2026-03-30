use crate::canvas::Canvas;

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

   pub fn get_block_size_and_mark(canvas: &mut Canvas, x: usize, y: usize) -> i64 {
    let colour_name = canvas[x][y].name();
    return fill(canvas, x, y, colour_name, 0)
}

pub fn set_block_size_at_marked(canvas: &mut Canvas, block_size: i64) {
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

// pub fn get_command(from_codel: Codel, to_codel: Codel) -> Box<dyn Fn(f64) -> f64> {
    
//     return Box::new(move |x: f64| x)
// }
