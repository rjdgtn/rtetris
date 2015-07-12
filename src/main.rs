//#![allow(dead_code)]
//#![allow(unused_imports)]
#![allow(non_snake_case)]
//#![allow(unused_variables)]

extern crate opengl_graphics;
extern crate glutin_window;
extern crate piston;
extern crate graphics;
extern crate vecmath;
extern crate rand;

mod logic;
mod render;
mod music;

use music::GameSound::GameSound;
use logic::figure::*;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::event::*;
use std::option::Option;
use graphics::*;
use piston::input::Button::*;
use piston::input::keyboard::*;

 
struct App {
	window: Option<Window>,
	gameSound: GameSound,
	game: logic::game::Game,
	rnd: render::renderer::Renderer,
	gameStepElapsed: f32
}

impl App {
	fn new() -> App {
		let gameSize: Vec2 = [9, 15];
		let cellSize = 15;
		
		let ws = WindowSettings::new(
        	"spinning-square",
        	[(gameSize[0] * cellSize) as u32, (gameSize[1] * cellSize) as u32]
    		);
		
		let wnd = Window::new(ws);
	
		App { window: Some(wnd)
			, game: logic::game::Game::new(gameSize)
			, gameSound: GameSound::new()	
			, rnd: render::renderer::Renderer::new(cellSize)
			, gameStepElapsed: 0.0
		}
	
	}

	fn run(mut self) {
		let opengl = OpenGL::_3_2;
		let mut gl = GlGraphics::new(opengl);
		
		self.gameSound.playMusic();
		let evs = self.window.take().unwrap().events();
		
		
		for e in evs {
			if let Some(r) = e.render_args() {
				self.render(&mut gl, r);
			}
		
	       if let Some(u) = e.update_args() {
	            self.update(u.dt);
	       } 
	         
	       if let Some(k) = e.press_args() {
	       	
	            match k {
	            	Keyboard(b) => {
	            		if self.game.state != logic::game::GameState::Finished {
		            		match b {
		            			Key::Space => { self.game.rotate(); }
		            			Key::Right => { self.game.moveFigure(true); }
		            			Key::Left => { self.game.moveFigure(false); }
		            			Key::Down => { self.game.forse(); }
		            			_ => { }
		            		}
	            		} else {
	            			self.game.restart();
	            		}
	            	}
	            	_ => { }
	            }
	       }
        }
	}
	
	fn render(&mut self, gl: &mut GlGraphics, r: RenderArgs) {	
		gl.draw(r.viewport(), |c, gl: &mut GlGraphics| {          
                clear([0.0, 0.0, 0.0, 1.0], gl);
                self.rnd.render(&c, gl, &self.game)
            });
	}
	
	fn update(&mut self, elapsed: f64) {
		let gameStepPeriod:f32 = 0.5;
		self.gameStepElapsed += elapsed as f32;;
		
		if self.gameStepElapsed > gameStepPeriod {
			self.gameStepElapsed = 0.0;
			self.game.playTurn();
		}
	}
}

fn main() { 
	let app = App::new();
	
	app.run();
	
    println!("Hello, world!");
}
