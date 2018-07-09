extern crate stdweb;
use stdweb::web::event;
mod game;
mod draw;
mod constants;

use game::Game;
use std::time::{Duration};
use std::thread::sleep;

// Shamelessly stolen from webplatform's TodoMVC example.
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}


fn main() {
    stdweb::initialize();
    let mut game = Game::new();

    loop {
        game.logic_step();
        sleep(Duration::from_millis(50));
        game.draw();
    }

//    stdweb::event_loop();
}
