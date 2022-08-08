use raylib::prelude::*;

use crate::{game::Game, loading::loading, menu::Menu, network::Network};

enum State {
    MainMenu,
    Loading,
    Game,
    PauseMenu,
}

pub struct GameState {
    debug: bool,
    state: State,
    network: Network,
    game: Option<Game>,
    menu: Menu,
}

impl GameState {
    pub fn new(debug: bool) -> Self {
        Self {
            state: State::MainMenu,
            debug,
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
            if self.debug {
                d.draw_fps(10, 10);
            }

            match self.state {
                State::MainMenu => {
                    self.menu.update(&mut d);
                    if self.menu.should_submit(&mut d) {
                        println!("JSON: [{}]", self.menu.to_json());
                        self.state = State::Loading;
                    }
                }
                State::Loading => loading(&mut d),
                State::Game => self.game(&mut d),
                State::PauseMenu => self.pause_menu(&mut d),
            }
        }
    }
}
