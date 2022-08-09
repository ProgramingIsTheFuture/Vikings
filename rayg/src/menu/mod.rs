use raylib::prelude::{Color, MouseCursor, RaylibDraw, RaylibDrawHandle};

use self::auth::Auth;

mod auth;

pub struct Menu {
    auth: Auth,
    retry: bool,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            auth: Auth::new(),
            retry: false,
        }
    }

    pub fn retry(&mut self) {
        self.retry = true;
        self.auth.reset();
    }

    pub fn should_submit(&mut self, d: &mut RaylibDrawHandle) -> bool {
        self.auth.is_auth(d)
    }

    pub fn to_json(&self) -> String {
        self.auth.to_json()
    }

    pub fn update(&mut self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::WHITE);

        if self.auth.should_write(d) {
            d.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_IBEAM);
            self.auth.writing(d);
        } else {
            d.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_DEFAULT);
        };

        self.auth.draw(d);
    }
}
