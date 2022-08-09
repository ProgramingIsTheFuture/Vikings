use raylib::{
    ffi::CheckCollisionPointRec,
    prelude::{Color, KeyboardKey, MouseButton, RaylibDraw, RaylibDrawHandle, Rectangle, Vector2},
};
use serde::Serialize;

use crate::constants::{HEIGHT, WIDTH};

#[derive(Serialize)]
struct Authentication {
    username: String,
}

impl Authentication {
    fn new() -> Self {
        Self {
            username: String::new(),
        }
    }
}

pub struct Auth {
    submit_rect: Rectangle,
    username_rect: Rectangle,
    authentication: Authentication,
    font_size: f32,
    max_length: i8,

    write: bool,
}

impl Auth {
    pub fn new() -> Self {
        let (w, h) = (WIDTH / 2.5 + 14.0, HEIGHT / 10.0);
        let rect = Rectangle::new((WIDTH / 2.0) - (w / 2.0), (HEIGHT / 2.0) + (h / 2.0), w, h);
        let font_size = HEIGHT / 14.0;
        let submit_rect = Rectangle::new(
            (WIDTH / 2.0) - (w / 2.0),
            (HEIGHT / 2.0) + (h / 2.0) + h + 15.0,
            w,
            h,
        );

        Self {
            max_length: 10,
            authentication: Authentication::new(),
            username_rect: rect,
            submit_rect,
            write: false,
            font_size,
        }
    }

    pub fn writing(&mut self, d: &mut RaylibDrawHandle) {
        let key_num = unsafe { raylib::ffi::GetCharPressed() };
        if key_num != 0 && self.authentication.username.len() < self.max_length as usize {
            if let Some(c) = char::from_u32(key_num as u32) {
                self.authentication.username.push(c.to_ascii_lowercase());
            }
        }

        if d.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
            self.authentication.username.pop();
        }
    }

    pub fn reset(&mut self) {
        self.authentication.username = String::new();
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

    pub fn is_auth(&mut self, d: &mut RaylibDrawHandle) -> bool {
        if self.authentication.username.is_empty() {
            return false;
        }

        if !d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            return false;
        }

        self.submit_rect
            .check_collision_point_rec(Vector2::new(d.get_mouse_x() as f32, d.get_mouse_y() as f32))
    }

    pub fn to_json(&self) -> String {
        match serde_json::to_string(&self.authentication) {
            Ok(v) => v,
            Err(_) => String::from(""),
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_rec(self.submit_rect, Color::BLUE);
        d.draw_rectangle_rec(self.username_rect, Color::BLUE);
        d.draw_text(
            "Enter!",
            self.submit_rect.x as i32 + (self.submit_rect.width / 4.0) as i32,
            self.submit_rect.y as i32 + (self.font_size as i32 / 4) + 2,
            self.font_size as i32,
            Color::BLACK,
        );

        if !self.authentication.username.is_empty() {
            d.draw_text(
                self.authentication.username.as_str(),
                self.username_rect.x as i32 + 2,
                self.username_rect.y as i32 + (self.font_size as i32 / 4),
                self.font_size as i32,
                Color::BLACK,
            );
        } else {
            d.draw_text(
                "Write here",
                self.username_rect.x as i32 + 2,
                self.username_rect.y as i32 + (self.font_size as i32 / 4),
                self.font_size as i32,
                Color::BLACK,
            );
        }
    }
}
