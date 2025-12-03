// use crossterm::event::{self, Event, KeyCode};
// use crossterm::execute; use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::execute; use crossterm::cursor::{Hide, Show};
// use std::time::Duration;
use std::io::stdout;
// use term_hopper::cls;

mod player;
mod scene;
mod data;
mod room;

use scene::{
    Scene
};
// use data::{
//     SaveData,
// };

#[derive(PartialEq)]
pub enum GameState {
    Menu(String),  // Contains title of the loaded menu
    Scene(String), // Contains title of the loaded scene
    Shutdown       // Time to gracefully exit execution
}

///
/// Handles the macro pieces of loading scenes, controlling visual display, and the core update loop
/// 
pub struct GameController {
    state: GameState,             // The current game state
    // TODO implement SaveData system
    // data: SaveData,               // The instance of the player's save data
}

impl GameController {
    ///
    /// Creates a new instance of the game controller
    /// 
    pub fn new() -> GameController {
        // let save = SaveData::load();
        GameController {
            state: GameState::Menu("Main".to_owned()),
            // TODO implement SaveData system
            // data: save,
        }
    }
    /// 
    /// Handles the macro execution loop of the game.
    /// NOTICE: Calling this command when the GameState is already a Scene will break things.
    /// 
    pub fn run(&mut self) -> Result<(), std::io::Error> {
        //         v---------+
        // Main menu -> exit |
        //  \/ /\            | 
        // Level select      |
        // \/                |   
        // Gameplay ---------+
        //
        loop {
            match &self.state {
                GameState::Menu(menu_name) => {
                    match menu_name.as_str() {
                        "Main" => {
                            self.main_menu();
                        }
                        _ => { }
                    }
                }
                GameState::Scene(scene_name) => {
                    let mut scene = Scene::load_from(scene_name.to_owned());
                    let _ = scene.play();
                }
                GameState::Shutdown => {
                    break;
                }
            }
        }

        return Ok(());
    }
}

// Menus
impl GameController {
    ///
    /// Displays and runs the main menu
    /// 
    pub fn main_menu(&mut self) {
        self.state = GameState::Scene(r"scenes/test-scene.json".to_owned());
    }
}

fn main() -> Result<(), std::io::Error> {
    // Prep the terminal
    execute!(stdout(), Hide)?;
    // Run the game
    let mut gc = GameController::new();
    let res = gc.run();
    // Cleanup terminal
    execute!(stdout(), Show)?;
    return res;
}