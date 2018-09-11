extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;


use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use Snake;

pub struct Game {
    pub gl : GlGraphics,
    pub snake : Snake::Snake,
}


impl Game {
    pub fn render(&mut self, arg: &RenderArgs){
        use graphics;
        let GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        self.gl.draw(arg.viewport(), |_ctx, gl|{
            graphics::clear(GREEN,gl);
        });

        self.snake.render(&mut self.gl, arg);
    }

    pub fn update(&mut self){
        self.snake.update();
    }

    pub fn press(&mut self, btn: &Button){
        let last_direction = self.snake.direction.clone();

        self.snake.direction = match btn{
            &Button::Keyboard(Key::Right)
                if last_direction != Snake::Direction::L => Snake::Direction::R,
            &Button::Keyboard(Key::Left)
                if last_direction != Snake::Direction::R => Snake::Direction::L,
            &Button::Keyboard(Key::Up)
                if last_direction != Snake::Direction::D => Snake::Direction::U,
            &Button::Keyboard(Key::Down)
                if last_direction != Snake::Direction::U => Snake::Direction::D,
            _ => last_direction,
        };

        if (btn == &Button::Keyboard(Key::NumPad1) || btn == &Button::Keyboard(Key::D1)){
            self.snake.body.push_back((-1,-1));
        }
        if (btn == &Button::Keyboard(Key::NumPad2) || btn == &Button::Keyboard(Key::D2)){
            if self.snake.body.len() > 1 {
                self.snake.body.pop_back();
            }
        }
    }

}
