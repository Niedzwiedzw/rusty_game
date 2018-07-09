use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{
    document,
    window,
    CanvasRenderingContext2d
};
use stdweb::web::html_element::CanvasElement;
use stdweb::web::event::{
    MouseMoveEvent,
    ResizeEvent,
};

use draw::draw_square;
use constants::{
    BOARD_SIZE,
};

fn get_canvas() -> CanvasElement {
    document().query_selector( "#canvas" )
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap()
}

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

#[derive(Copy, Clone, Debug)]
pub enum BoardCell {
    Snake,
    Apple,
    Wall,
    Blank,
}
type GameBoard = [[BoardCell; BOARD_SIZE]; BOARD_SIZE];

pub struct Game {
    canvas: CanvasElement,
    context: CanvasRenderingContext2d,
    board: GameBoard,
    cell_size: u32,
}

impl Game {
    pub fn new() -> Self {
        let (canvas, context) = setup();
        let cell_size = ((canvas.offset_width() + canvas.offset_height())/2) as u32;
        Game {
            canvas,
            context,
            board: [[BoardCell::Blank; BOARD_SIZE]; BOARD_SIZE],
            cell_size
        }
    }

    fn draw_cell(&self, x: u32, y:u32, cell: &BoardCell) {
        draw_square(&self.context, x, y, self.cell_size, cell);
    }

    pub fn logic_step(&mut self) {
        let enums = vec![BoardCell::Apple, BoardCell::Snake];
        let new_color = match &self.board[0][0] {
            &BoardCell::Snake => &BoardCell::Apple,
            _ => &BoardCell::Snake
        };
        self.board = [[*new_color; BOARD_SIZE]; BOARD_SIZE];
    }

    pub fn draw(&self) {
        for (y, row) in self.board.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                self.draw_cell(x as u32, y as u32, cell);
            }
        }
    }
}

fn setup() -> (CanvasElement, CanvasRenderingContext2d) {

    let canvas: CanvasElement = get_canvas();
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

    window().add_event_listener( enclose!( (canvas) move |_: ResizeEvent| {
        canvas.set_width(canvas.offset_width() as u32);
        canvas.set_height(canvas.offset_height() as u32);
    }));

    canvas.set_width(canvas.offset_width() as u32);
    canvas.set_height(canvas.offset_height() as u32);

    (canvas, context)
}
