extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;


use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use std::collections::LinkedList;
use std::iter::FromIterator;

mod Game;
mod Snake;

fn main() {
    let opengl = OpenGL::V3_0;

    let mut window:GlutinWindow = WindowSettings::new(
        "Snake",
        [600,400],
    ).opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::Game{
        gl: GlGraphics::new(opengl),
        snake: Snake::Snake{
            body : LinkedList::from_iter((vec![(0,0), (0,1), (0,2),(0,3), (0,4)]).into_iter()),
            direction : Snake::Direction::R,
        },
    };


    let mut events = Events::new(EventSettings::new()).ups(5);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
             game.update();
        }
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.press(&k.button);
            }
        }
    }
}
