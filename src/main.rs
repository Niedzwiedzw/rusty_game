use std::rc::Rc;
use std::cell::{RefCell};
//use std::borrow::BorrowMut;

extern crate stdweb;
use stdweb::web;

mod game;
mod draw;
mod constants;


use game::Game;

// Shamelessly stolen from webplatform's TodoMVC example.
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

fn main_loop(game: Rc<RefCell<Game>>) {
//    game.logic_step();
//    game.draw();
//    main_loop(game);
    game.borrow_mut().logic_step();
    game.borrow().draw();
    web::window().request_animation_frame(move |_| {
        main_loop(game);
    });
}

fn main() {
    stdweb::initialize();
    let game =  Rc::new(RefCell::new(Game::new()));

    main_loop(game);

//    stdweb::event_loop();
}
