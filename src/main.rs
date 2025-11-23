use crossterm::event::{self, Event, KeyCode};
use crossterm::execute; use crossterm::cursor::{MoveTo, Hide, Show};

use std::time::Duration;
use std::io::stdout;

use term_hopper::cls;

mod player;
use player::{
    Player, Direction
};
mod level;
use level::{
    Level
};

fn main() -> Result<(), std::io::Error> {
    // Prep the terminal
    execute!(stdout(), Hide)?;

    // Load level data and show level
    let level = Level::load_from("levels\\test-bed.txt".to_owned());
    level.display();

    // Create and display player
    let mut player = Player::new(Direction::North, level.player_start);
    player.display();

    // Input loop
    loop {
        if event::poll(Duration::from_millis(500))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.is_press() {
                        match key.code {
                            KeyCode::Char(c) => {
                                if player.is_typing {
                                    player.handle_typing(c);
                                } else {
                                    player.is_typing = true;
                                    player.handle_typing(c);
                                }
                            }
                            KeyCode::Enter => {
                                if player.is_typing {
                                    player.is_typing = false;
                                } else {
                                    // TODO Use enter as key to enter rooms?
                                }
                            }
                            KeyCode::Esc => { break; }
                            KeyCode::Up | KeyCode::Down | KeyCode::Right | KeyCode::Left => {
                                player.move_player(&level, key.code);
                            }
                            _ => { }
                        }
                    }
                }
                Event::Resize(x, y) => {
                    // TODO determine min screen size by level size
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

    // Cleanup
    execute!(stdout(), Show)?;
    cls();
    execute!(stdout(), MoveTo(0, 0))?;
    Ok(())
}