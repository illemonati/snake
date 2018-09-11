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

pub struct Snake {
    // pub pos_x : i32,
    // pub pos_y : i32,
    pub direction : Direction,
    pub body: LinkedList<(i32,i32)>,
}

impl Snake {
    pub fn render(&self, gl: &mut GlGraphics, args : &RenderArgs){
        use graphics;
        let RED: [f32; 4] = [1.0,0.0,0.0,1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x,y)|{
                graphics::rectangle::square(
                    (x * 20).into(),
                    (   y * 20).into(),
                    20f64)
            })
            .collect();


        gl.draw(args.viewport(), |ctx, gl|{
            let transform = ctx.transform;
            squares.into_iter()
                .for_each(|square| graphics::rectangle(RED, square, transform, gl));
        });
    }

    pub fn update(&mut self){
        let mut new_head = (*self.body.front().expect("Snake does not have a body !!!")).clone();
        // match self.direction{
        //     Direction::R => self.pos_x += 1,
        //     Direction::L => self.pos_x -= 1,
        //     Direction::U => self.pos_y -= 1,
        //     Direction::D => self.pos_y += 1,
        // }
        match self.direction{
            Direction::R => new_head.0 += 1,
            Direction::L => new_head.0 -= 1,
            Direction::U => new_head.1 -= 1,
            Direction::D => new_head.1 += 1,
        }
        self.body.push_front(new_head);
        self.body.pop_back().unwrap();
    }
}

#[derive(Clone, PartialEq)]
pub enum Direction {
    R, L, U, D,
}
