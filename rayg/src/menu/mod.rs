use raylib::prelude::{Color, MouseCursor, RaylibDraw, RaylibDrawHandle};

use self::auth::Auth;

mod auth;

pub struct Menu {
    auth: Auth,
}

impl Menu {
    pub fn new() -> Self {
        Self { auth: Auth::new() }
    }

    pub fn update(&mut self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::WHITE);

        if self.auth.should_write(d) {
            d.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_IBEAM);
            self.auth.writing();
        } else {
            d.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_DEFAULT);
        };

        d.draw_text(self.auth.username.as_str(), 12, 12, 20, Color::BLACK);

        d.draw_rectangle_rec(self.auth.username_rect, Color::BLUE);
    }
}
