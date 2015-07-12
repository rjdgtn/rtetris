use logic::game::*;

use opengl_graphics::{ GlGraphics };
use graphics::{ Context, DrawState};
use graphics::rectangle::*;


pub struct Renderer {
	cellSize: f64,
	rectangle: Rectangle,
//	cellTexture : Texture,
//	cellImage: Image,
}

impl Renderer {
	pub fn new(cellSize: isize)->Renderer {
		let b = Border{
			color: [0.0, 0.0, 0.0, 1.0],
			radius: 1.0
			};
		let rect = Rectangle::new([1.0, 1.0, 1.0, 1.0]).border(b);
		Renderer {
			cellSize: cellSize as f64,
			rectangle: rect,
//			cellTexture: Texture::from_path(Path::new("rust-2048.png")).unwrap(),
//			cellImage: Image::new().rect(square(0.0, 0.0, 10.0)),
		}
	}
	
	pub fn render(&mut self, coxt:&Context, gl: &mut GlGraphics, game:&Game) {
	
		let board = &game.board;
		for i in 0..board.size[0] {
			for j in 0..board.size[1] {
				if board.occuped([i, j]) > 0 {
					let xpos: f64 = i as f64 * self.cellSize;
					let ypos: f64 = j as f64 * self.cellSize;
					self.rectangle.draw([xpos, ypos, self.cellSize, self.cellSize], &DrawState::new(), coxt.transform, gl);
				}
			}	
		}
	
		
		match game.figure.as_ref() {
			Some(f) => {
				for i in 0..f.1.size[0] {
					for j in 0..f.1.size[1] {
						if f.1.occuped([i,j]) > 0 {
							let xpos: f64 = (i as isize + f.0[0]) as f64 * self.cellSize;
							let ypos: f64 = (j as isize + f.0[1]) as f64 * self.cellSize;
							self.rectangle.draw([xpos, ypos, self.cellSize, self.cellSize], &DrawState::new(), coxt.transform, gl);
						}
					}
				}
			}
			_ => { }
		}
	}
} 