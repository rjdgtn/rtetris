use rand::random;

#[derive(Debug)]
pub enum FigureType {
	I = 0,
	J = 1,
	L = 2,
	O = 3,
	S = 4,
	T = 5,
	Z = 6,
	NUM = 7
}

use vecmath::*;

pub type Vec2 = Vector2<isize>;

pub fn posToIdx(pos:Vec2, size:Vec2) -> usize {
	 	(pos[0] * size[1] + pos[1]) as usize
}

#[derive(Clone)]
pub struct Figure {
	pub size: Vec2,
	pub mask: [u8; 12],
}

impl Figure {
	pub fn new(ftype : FigureType) -> Figure {
		println!("{:?}", ftype);
	 	match ftype {
		    FigureType::I => Figure{ size:[3, 4], mask:[0, 0, 0, 0,
		    	                                        1, 1, 1, 1,
		    	                                        0, 0, 0, 0] },
		    FigureType::J => Figure{ size:[2, 3], mask:[1, 0, 0,
		    	                                        1, 1, 1,
		    	                                        0, 0, 0, 0, 0, 0] },
		    FigureType::L => Figure{ size:[2, 3], mask:[0, 0, 1,
		    	                                        1, 1, 1,
		    	                                        0, 0, 0, 0, 0, 0] },
		    FigureType::O => Figure{ size:[2, 2], mask:[1, 1, 
		    	                                        1, 1,
		    	                                        0, 0, 0, 0, 0, 0, 0, 0] },
		    FigureType::S => Figure{ size:[2, 3], mask:[0, 1, 1,
		    	                                        1, 1, 0,
		    	                                        0, 0, 0, 0, 0, 0] },
		    FigureType::T => Figure{ size:[2, 3], mask:[0, 1, 0,
		    	                                        1, 1, 1,
		    	                                        0, 0, 0, 0, 0, 0] },
		    FigureType::Z => Figure{ size:[2, 3], mask:[1, 1, 0,
		    	                                        0, 1, 1,
		    	                                        0, 0, 0, 0, 0, 0] },
		    FigureType::NUM => panic!("wrong figure type")
		}
	 }
	 
	 pub fn new_random() -> Self {
	 	let i = random::<usize>() % FigureType::NUM as usize;
	 	
		if i == FigureType::I as usize { return Figure::new(FigureType::I); }
		else if i == FigureType::J as usize { return Figure::new(FigureType::J); }
		else if i == FigureType::L as usize { return Figure::new(FigureType::L); }
		else if i == FigureType::O as usize { return Figure::new(FigureType::O); }
		else if i == FigureType::S as usize { return Figure::new(FigureType::S); }
		else if i == FigureType::T as usize { return Figure::new(FigureType::T); }
		else if i == FigureType::Z as usize { return Figure::new(FigureType::Z); }
		else { panic!("wrong type idx"); }
	 }
	 
	 pub fn rotate(&mut self) {
	 	let old_mask = self.mask;
	 	let old_size = self.size;
	 	self.size = [self.size[1], self.size[0]];
	 	for i in 0..self.size[0] {
	 		for j in 0..self.size[1] {
	 			let dstPos = posToIdx([i, j], self.size); 
	 			let srcPos = posToIdx([j, old_size[1]-1-i], old_size);
	 			self.mask[dstPos] = old_mask[srcPos];
	 		}
	 	}
	 } 
	 
	 pub fn occuped(&self, pos:Vec2) -> u8 {
	 	self.mask[posToIdx(pos, self.size)]
	 }
}

pub struct Board {
	pub size: Vec2,	
	pub cells: Vec<u8>,
}

impl Board {
	pub fn new(sz : Vec2) -> Board {
		Board { size:sz, cells:vec![0; (sz[0] * sz[1]) as usize] }
	}
	
	pub fn occuped(&self, pos:Vec2) -> u8 {
		let idx = posToIdx(pos, self.size);
	 	self.cells[idx]
	}
	
	pub fn checkPlace(&self, fg:&Figure, pos:Vec2) -> bool {
		for i in 0..fg.size[0] {
			for j in 0..fg.size[1] {
				if fg.occuped([i,j]) > 0 {
					let boardPos = vec2_add(pos, [i,j]);
					
					if boardPos[0] < 0 || boardPos[1] < 0 {
						return false;
					}
					
					if boardPos[0] >= self.size[0] || boardPos[1] >= self.size[1] {
						return false;
					}
						
					if self.occuped(boardPos) > 0 {
					   return false;
				    }
				}
			}
		}
		
		true
	}
	
	pub fn fix(&mut self, fg:&Figure, pos:Vec2)  {
		println!(" fix at {}, {}", pos[0], pos[1]);
		for i in 0..fg.size[0] {
			for j in 0..fg.size[1] {
				let boardPos = vec2_add(pos.clone(), [i,j]);
				assert!(boardPos[0] < self.size[0] || boardPos[1] < self.size[1], "out of board");
				if fg.occuped([i,j]) > 0 {
					let idx = posToIdx(boardPos, self.size);
					
					println!("\tfix {}", idx);
				    self.cells[idx] = fg.occuped([i,j])
			    }
			}
		}
	}
	
	/// return num erased lines
	pub fn eraseLines(&mut self) -> isize {
		let mut numLines = 0;
		
		let mut j = self.size[1]-1;
		while j >= 0 {
			let mut hasSpaces = false;
			for i in 0..self.size[0] {
				if self.occuped([i,j]) == 0 {
					hasSpaces = true;
					break;
				}
			}
			
			if !hasSpaces {
				numLines += 1;
				self.eraseLine(j);
			} else {
				j -= 1;
			}
		} 
		
		return numLines;
	}
	
	pub fn eraseLine(&mut self, line: isize) {
		let mut j = line;
		while j >= 0 {
			for i in 0..self.size[0] {
				if j <= 0 {
					self.cells[posToIdx([i, j], self.size)] = 0;
				} else {
					self.cells[posToIdx([i, j], self.size)] = self.occuped([i, j-1]);
				}
			}
			j -= 1;	
		}
	}
}