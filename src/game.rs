use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{
    document,
    window,
    CanvasRenderingContext2d,
    Date
};
use stdweb::web::html_element::CanvasElement;
use stdweb::web::event::{
    MouseMoveEvent,
    ResizeEvent,
};

use std::collections::VecDeque;

use draw::draw_square;
use constants::{
    BOARD_SIZE,
    GAME_TICK
};

fn get_canvas() -> CanvasElement {
    document().query_selector( "#canvas" )
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap()
}

pub fn current_timestamp() -> f64 {
    Date::new().value_of()
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

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    head_x: usize,
    head_y: usize,
    direction: Direction,
    tail: VecDeque<(usize, usize)>,
    length: usize,
}

impl Snake {
    pub fn new() -> Self {
        let mut snake = Snake {
            head_x: (BOARD_SIZE/2),
            head_y: (BOARD_SIZE/2),
            direction: Direction::Up,
            tail: VecDeque::new(),
            length: 2
        };
        snake.tail.push_back((snake.head_x, snake.head_y));
        snake
    }

    pub fn step(&mut self) {
        match self.direction {
            Direction::Up => {self.head_y -= 1;},
            Direction::Down => {self.head_y += 1;},
            Direction::Left => {self.head_x -= 1;},
            Direction::Right => {self.head_x += 1;},
            _ => {},
        };
        self.tail.push_back((self.head_x, self.head_y));
        if self.tail.len() > self.length {
            self.tail.pop_front();
        }
    }
}

pub struct Game {
    canvas: CanvasElement,
    context: CanvasRenderingContext2d,
    board: GameBoard,
    cell_size: u32,
    next_tick: f64,
    snake: Snake,
}

impl Game {
    pub fn new() -> Self {
        let (canvas, context) = setup();
        let cell_size = (((canvas.offset_width() + canvas.offset_height())/2)/BOARD_SIZE as i32) as u32;
        Game {
            canvas,
            context,
            board: [[BoardCell::Blank; BOARD_SIZE]; BOARD_SIZE],
            cell_size,
            next_tick: 0.0,
            snake: Snake::new()
        }
    }

    fn draw_cell(&self, x: u32, y:u32, cell: &BoardCell) {
        draw_square(&self.context, x, y, self.cell_size/2, cell);
    }

    pub fn logic_step(&mut self) {
        console!(log, "{}", current_timestamp());
        if !(self.next_tick < current_timestamp()) {
            return;  // too soon
        }

        // reset the board
        let mut board: GameBoard = [[BoardCell::Blank; BOARD_SIZE]; BOARD_SIZE];
        self.snake.step();

        for (cell_x, cell_y) in &self.snake.tail {
            board[*cell_x][*cell_y] = BoardCell::Snake;
        }

        self.board = board;
        // plan the next tick
        self.next_tick = current_timestamp() + GAME_TICK;
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
