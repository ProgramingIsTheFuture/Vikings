use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle};

pub fn loading(d: &mut RaylibDrawHandle) {
    d.clear_background(Color::BLACK);

    d.draw_text("Loading!...", 50, 50, 40, Color::WHITE);
}
