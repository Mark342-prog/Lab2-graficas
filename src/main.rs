mod line;
mod framebuffer;
mod conway;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use conway::GameOfLife;
use std::thread;
use std::time::Duration;

fn main() {
    let grid_width = 1000;
    let grid_height = 1000;
    let cell_size = 1;
    let window_width = grid_width;
    let window_height = grid_height;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width as i32, window_height as i32)
        .title("Conway's Game of Life")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(grid_width as u32, grid_height as u32);
    framebuffer.set_background_color(Color::BLACK);
    let mut game_of_life = GameOfLife::new(grid_width, grid_height);
    
    let mut frame_count = 0;
    let frames_per_update = 2;
    
    while !window.window_should_close() {
        if window.is_key_pressed(KeyboardKey::KEY_SPACE) {
            frame_count = 0;
        }
        
        if window.is_key_pressed(KeyboardKey::KEY_R) {
            game_of_life.reset();
        }
        
        if window.is_key_pressed(KeyboardKey::KEY_C) {
            game_of_life.clear();
        }
        
        if window.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_pos = window.get_mouse_position();
            let cell_x = (mouse_pos.x / cell_size as f32) as usize;
            let cell_y = (mouse_pos.y / cell_size as f32) as usize;
            game_of_life.toggle_cell(cell_x, cell_y);
        }
        
        frame_count += 1;
        if frame_count >= frames_per_update {
            game_of_life.update();
            frame_count = 0;
        }
        
        framebuffer.clear();
        game_of_life.render(&mut framebuffer);
        framebuffer.swap_buffers(&mut window, &raylib_thread);
        thread::sleep(Duration::from_micros(6944));
    }
}



