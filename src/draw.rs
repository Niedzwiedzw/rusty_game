use stdweb::web::CanvasRenderingContext2d;
use game::BoardCell;
use constants;


fn color_of<'a>(cell: &'a BoardCell) -> &'a str {
    match cell {
        &BoardCell::Snake => constants::OLIVE,
        &BoardCell::Apple => constants::RED,
        &BoardCell::Wall => constants::BLACK,
        &BoardCell::Blank => constants::SILVER,
        _ => panic!("Internal error!!!"),
    }
}

pub fn draw_square(context: &CanvasRenderingContext2d, x: u32, y: u32, size: u32, cell: &BoardCell) {
    context.set_fill_style_color(color_of(cell));
    context.fill_rect(
        (x as f64)*7.0,
        (y as f64)*7.0,
        size as f64,
        size as f64,
    );
}
