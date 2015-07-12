
#[cfg(not(feature = "music-in-game"))]
pub mod GameSound {
	
	pub struct GameSound ;
	
	impl GameSound {
		pub fn new() -> GameSound {
			GameSound
		}
		
		pub fn playMusic(&mut self) {
		}
	}
}

#[cfg(feature = "music-in-game")]
pub mod GameSound {
	extern crate ears;
	
	use self::ears::Music;
	use self::ears::Sound;
	use self::ears::AudioController; 
	
	pub struct GameSound {
		music: Sound
	}
	
	impl GameSound {
		pub fn new() -> GameSound {
			
			let mut music = Sound::new("res/korobeiniki.ogg").unwrap();
			music.set_looping(true);
		
			GameSound { music:music }
		}
		
		pub fn playMusic(&mut self) {
			self.music.play();
		}
	}
}