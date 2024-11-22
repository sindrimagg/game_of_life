use std::env;
use std::thread::sleep;
use std::time::Duration;

use game_of_life::Game;

fn main() {
    let mut game = Game::new(&env::args().collect(), (25, 40), true);

    let mut i = 0;

    loop {
        game.print();
        i = i + 1;
        println!("{i}");
        sleep(Duration::from_millis(200));

        game.update();
    }
}
