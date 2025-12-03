
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

///
/// Contains general information about a screen of map in gameplay
/// 
#[derive(Serialize, Deserialize)]
pub struct Room {
    pub tilemap: String,                                  // Visual characters that make up the map
    pub transitions: HashMap<(u16, u16), TransitionInfo>, // Transitions - representation of connections between rooms. 
    pub solid_tiles: Vec<char>                            // Contains a list of characters that are solid to the player
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TransitionInfo {
	pub id: String,    // Room ID of the destination
	pub x: u16,        // X coordinate destination
	pub y: u16,        // Y coordinate destination
}

// Object checks
impl Room {
	/// 
	/// Check if a given point in a room is a transition. If so, return true
	/// 
	pub fn is_transition(&self, x: u16, y:u16) -> bool {
		for key in self.transitions.keys() {
			if key.0 == x && key.1 == y {
				return true;
			}
		}
		return false;	
	} 
	///
	/// Check if a given point in a room is solid. If so, return true.
	/// 
	pub fn is_solid(&self, x: u16, y: u16) -> bool {
		let row: String = self.tilemap.split("\n").collect::<Vec<&str>>()[y as usize].to_owned();
		let col: char = row.chars().collect::<Vec<char>>()[x as usize];
		for c in self.solid_tiles.iter() {
			if col == *c {
				return true;
			}
		}
		return false;
	}
}

// Info accessors
impl Room {
	///
	/// Get the info about a transition. Assumes a valid transition location is passed.
	/// Meant for use only after finding a transition with `.is_transition()`.
	/// 
	pub fn get_transition_info(&self, x: u16, y: u16) -> TransitionInfo {
		return self.transitions.get(&(x, y)).unwrap().clone();
	}
}