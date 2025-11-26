
use std::collections::HashMap;

use crate::player::Direction;

///
/// Contains general information about a screen of map in gameplay
/// 
pub struct Room {
    pub tilemap: String,                  // Visual characters that make up the map
    pub transitions: HashMap<(u16, u16),  // Transitions - representation of connections between rooms.
      (String, u16, u16, Direction)>,     // (trigger_x, trigger_y), (dest_room, start_x, start_y, direction)
    
}