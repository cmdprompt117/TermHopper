use std::collections::HashMap;

use crate::room::Room;
use crate::player::Player;

//
/// Contains general scene data read from a .txt descriptor file
/// 
pub struct Scene {
    room_map: HashMap<String, Room>, // Contains all rooms related to a scene, keys = room IDs
    current_room: Room,              // The room currently displayed
    player: Player,                  // The player controller moving between rooms
}

impl Scene {
    ///
    /// Load a Scene object from definition in .json file
    /// 
    pub fn load_from(file_path: String) -> Scene {
        todo!();
    }
    ///
    /// Play out the Scene - this is where the player input loop is
    /// 
    pub fn play(&mut self) {

    }
}