use std::collections::HashMap;
use std::fs;
use std::time::Duration;
use std::io::stdout;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    cursor::{MoveTo},
};
use serde::{Serialize, Deserialize};
use serde_json;

use crate::room::{Room, TransitionInfo};
use crate::player::Player;

//
/// Contains general scene data read from a .txt descriptor file
/// 
#[derive(Serialize, Deserialize)]
pub struct Scene {
    room_map: HashMap<String, Room>, // Contains all rooms related to a scene, keys = room IDs
    current_room: Room,              // The room currently displayed
    player: Player,                  // The player controller moving between rooms
}

impl Scene {
    ///
    /// Load a Scene object from a `.json` file
    /// 
    pub fn load_from(file_path: String) -> Scene {
        let scene_str = fs::read_to_string(&file_path).expect(format!("File not found while loading scene: {}", file_path).as_str());
        return serde_json::from_str(&scene_str).expect(format!("Invalid scene definition in {}", file_path).as_str());
    }
    ///
    /// Play out the Scene - this is where the player input loop is
    /// 
    pub fn play(&mut self) -> Result<(), std::io::Error> {
        // 0. Visually load the scene
        self.show_room();
        // 1. Begin player input loop
        loop {
            if event::poll(Duration::from_millis(500))? {
                match event::read()? {
                    Event::Key(key) => {
                        if key.is_press() {
                            match key.code {
                                KeyCode::Char(c) => {
                                    if self.player.is_typing {
                                        self.player.handle_typing(c);
                                    } else {
                                        self.player.is_typing = true;
                                        self.player.handle_typing(c);
                                    }
                                }
                                KeyCode::Enter => {
                                    if self.player.is_typing {
                                        self.player.is_typing = false;
                                    }
                                }
                                KeyCode::Esc => { break Ok(()); }
                                KeyCode::Up | KeyCode::Down | KeyCode::Right | KeyCode::Left => {
                                    // If res returns something, we need to handle a transition
                                    let res = self.player.move_player(&self.current_room, key.code);
                                    if res.is_some() {
                                        self.handle_transition(res.unwrap());
                                    }
                                }
                                _ => { }
                            }
                        }
                    }
                    Event::Resize(x, y) => {
                        // TODO determine min screen size by scene size
                        // TODO and handle re-printing the screen when the needed size is met
                        if x < 20 || y < 20 {
                            execute!(stdout(), MoveTo(0, 0))?;
                            term_hopper::cls();
                            println!("Please resize your terminal to at least 20x20.");
                        }
                    }
                    _ => { }
                }
            }
        }
    }
    ///
    /// Handles the player transitioning to another screen
    /// 
    pub fn handle_transition(&self, _dest: TransitionInfo) {
        todo!();
    }
}

// Helpers
impl Scene {
    ///
    /// Reset screen visuals and display the current room
    /// 
    pub fn show_room(&self) {
        term_hopper::cls();
        println!("{}", self.current_room.tilemap);
    }
}