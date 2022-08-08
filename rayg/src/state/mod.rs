use raylib::prelude::*;

use crate::{game::Game, menu::Menu, network::Network};

enum State {
    MainMenu,
    Game,
    PauseMenu,
}

pub struct GameState {
    state: State,
    network: Network,
    game: Option<Game>,
    menu: Menu,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            state: State::MainMenu,
            network: Network::new(),
            game: None,
            menu: Menu::new(),
        }
    }

    fn game(&mut self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::BLACK);
        d.draw_text("Game mode!", 12, 12, 20, Color::WHITE);
    }

    fn pause_menu(&mut self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::BLUE);
        d.draw_text("Pause menu!", 12, 12, 20, Color::WHITE);
    }

    pub fn run(&mut self) {
        let (mut rl, thread) = init().size(800, 800).build();

        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);

            match self.state {
                State::MainMenu => self.menu.update(&mut d),
                State::Game => self.game(&mut d),
                State::PauseMenu => self.pause_menu(&mut d),
            }
        }
    }
}
