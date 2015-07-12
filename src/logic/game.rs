//extern crate vecmath;

//mod figure;
use logic::figure::*;
//use std::*;
 
#[derive(PartialEq)]
pub enum GameState {
	Prepare,
	Started,
	Finished,
}

pub struct Game {
	pub state: GameState,
	pub figure: Option<(Vec2, Figure)>,
	pub board: Board,
	pub score: isize
}

impl Game {
	pub fn new(sz:Vec2) -> Game {
		Game {
			state: GameState::Prepare,
			figure: None,
			board: Board::new(sz),
			score: 0,
		}
	}
	
	pub fn restart(&mut self) {
		self.board = Board::new(self.board.size);
		self.score = 0;
		self.state = GameState::Prepare;
		self.figure = None;
	}
	
	pub fn playTurn(&mut self) {
		match self.state {
		    GameState::Prepare => {
		    	self.state = GameState::Started;
		    	self.playTurn();
		    	},
		    GameState::Finished => { return; },
		    GameState::Started => {
		    	match self.figure {
		    		None => { 
		    			let figure = Figure::new_random();
		    			let f = ([(self.board.size[0] - figure.size[0])/2, 0], figure);
		    			if !self.board.checkPlace(&f.1, f.0) {
		    				self.state = GameState::Finished;
		    				println!("finish");
	    				}
		    			
		    			self.figure = Some(f);
		    			
	  					println!("create new");
		    			return;
	    			},
		    		_ => {  },
		    	};
		    
		    	match self.figure.clone() {
		    		Some(f) => {
		    			let newpos = [f.0[0], f.0[1] + 1];
		    			if !self.board.checkPlace(&f.1, newpos) {
		    				self.fixFigure();
		    			} else {
		    				self.figure.as_mut().unwrap().0 = newpos;
		    			}
	    			},
		    		_ => {  },
		    	};
		    	
	    	},
		}
	} 
	
	fn fixFigure(&mut self) {
		match self.figure.clone() {
			Some(f) => {
				self.board.fix(&f.1, f.0);
				self.figure = None;
				println!("fix");
				
		    	self.checkForLines();
			}
			_ => {  },
		}
	}
	
	pub fn rotate(&mut self) {
		match self.figure.as_mut() {
	    		Some(f) => {
	    			let mut fCopy = f.1.clone();
	    			fCopy.rotate();
	    			if self.board.checkPlace(&fCopy, f.0) {
    					f.1 = fCopy;
	    			}
				},
	    		_ => {  },
	    	};
	}
	
	pub fn moveFigure(&mut self, right: bool) {
		match self.figure.as_mut() {
	    		Some(f) => {
	    			let mut posCopy = f.0.clone();
	    			posCopy[0] += if right { 1 } else { -1 };
	    			if self.board.checkPlace(&f.1, posCopy) {
    					f.0 = posCopy;
	    			}
				},
	    		_ => {  },
	    	};
	}
	
	pub fn forse(&mut self) {
		match self.figure.clone() {
	    		Some(mut f) => {
	    			for _ in 0..self.board.size[1] {
		    			f.0[1] += 1;
		    			if !self.board.checkPlace(&f.1, f.0) {
		    				f.0[1] -= 1;
		    				self.figure = Some(f);
	    					self.fixFigure();
	    					break; 
		    			} 
	    			}
				},
	    		_ => {  },
	    	};
	}
	
	fn checkForLines(&mut self) {
		let numErasedLines = self.board.eraseLines();
		self.score == numErasedLines * 100;
	}
}