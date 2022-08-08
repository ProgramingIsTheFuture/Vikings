use raylib::prelude::{MouseButton, RaylibDrawHandle, Rectangle};

use crate::constants::{HEIGHT, WIDTH};

pub struct Auth {
    pub username_rect: Rectangle,
    pub username: String,

    write: bool,
}

impl Auth {
    pub fn new() -> Self {
        let (w, h) = (WIDTH / 2.5, HEIGHT / 10.0);
        let rect = Rectangle::new((WIDTH / 2.0) - (w / 2.0), (HEIGHT / 2.0) + (h / 2.0), w, h);

        Self {
            username: String::new(),
            username_rect: rect,
            write: false,
        }
    }

    pub fn writing(&mut self) {
        let key_num = unsafe { raylib::ffi::GetCharPressed() };
        if key_num != 0 {
            if let Some(c) = char::from_u32(key_num as u32) {
                self.username.push(c);
            }
        }
    }

    pub fn should_write(&mut self, d: &mut RaylibDrawHandle) -> bool {
        let x_mouse = d.get_mouse_x();
        let y_mouse = d.get_mouse_y();

        if !d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            return self.write;
        }

        self.write = (self.username_rect.x <= x_mouse as f32)
            && (x_mouse as f32 <= (self.username_rect.x + self.username_rect.width))
            && (self.username_rect.y <= y_mouse as f32)
            && (y_mouse as f32 <= (self.username_rect.y + self.username_rect.height));
        self.write
    }
}
