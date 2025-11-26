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
use data::{
    SaveData,
};

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
    data: SaveData,               // The instance of the player's save data
}

impl GameController {
    ///
    /// Creates a new instance of the game controller
    /// 
    pub fn new() -> GameController {
        let save = SaveData::load();
        GameController {
            state: GameState::Menu("Main".to_owned()),
            data: save,
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
                    scene.play();
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

    // Load scene data and show scene
//     let scene = Scene::load_from("scenes\\test-bed.txt".to_owned());
//     scene.display();

    // Create and display player
//     let mut player = Player::new(Direction::North, scene.player_start);
//     player.display();

    // Input loop
//     loop {
//         if event::poll(Duration::from_millis(500))? {
//             match event::read()? {
//                 Event::Key(key) => {
//                     if key.is_press() {
//                         match key.code {
//                             KeyCode::Char(c) => {
//                                 if player.is_typing {
//                                     player.handle_typing(c);
//                                 } else {
//                                     player.is_typing = true;
//                                     player.handle_typing(c);
//                                 }
//                             }
//                             KeyCode::Enter => {
//                                 if player.is_typing {
//                                     player.is_typing = false;
//                                 } else {
                                    // TODO Use enter as key to enter rooms?
//                                 }
//                             }
//                             KeyCode::Esc => { break; }
//                             KeyCode::Up | KeyCode::Down | KeyCode::Right | KeyCode::Left => {
//                                 player.move_player(&scene, key.code);
//                             }
//                             _ => { }
//                         }
//                     }
//                 }
//                 Event::Resize(x, y) => {
                    // TODO determine min screen size by scene size
                    // TODO and handle re-printing the screen when the needed size is met
//                     if x < 20 || y < 20 {
//                         execute!(stdout(), MoveTo(0, 0))?;
//                         term_hopper::cls();
//                         println!("Please resize your terminal to at least 20x20.");
//                     }
//                 }
//                 _ => { }
//             }
//         }
//     }

    // Cleanup
//     execute!(stdout(), Show)?;
//     cls();
//     execute!(stdout(), MoveTo(0, 0))?;
//     Ok(())
// }