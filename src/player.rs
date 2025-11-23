use crossterm::execute; use crossterm::cursor::MoveTo;
use crossterm::event::KeyCode;
use std::io::stdout;

use term_hopper::print_flush;
use crate::Level;

#[derive(PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West
}

pub struct Player {
    pub facing: Direction,
    pub x: u16,
    pub y: u16,
    
    pub is_typing: bool,
}

impl Player {
    ///
    /// Create new instance
    /// 
    pub fn new(start_dir: Direction, start_pos: (u16, u16)) -> Player {
        Player {
            facing: start_dir,
            x: start_pos.0,
            y: start_pos.1,
            is_typing: false
        }
    }
    ///
    /// Update character direction. Display accordingly
    /// 
    pub fn turn(&mut self, new_dir: Direction) {
        if self.facing != new_dir {
            self.facing = new_dir;
            self.display();
        }
    }
    ///
    /// Move the player and turn sprite based on input
    /// 
    pub fn move_player(&mut self, level: &Level, input: KeyCode) {
        match input {
            KeyCode::Up => {
                if !level.is_solid(self.x, self.y - 1) {
                    execute!(stdout(), MoveTo(self.x, self.y)).ok();
                    print_flush(level.get_char(self.x, self.y));
                    self.y -= 1;
                    self.display();
                }
                self.turn(Direction::North);
            }
            KeyCode::Right => {
                if !level.is_solid(self.x + 1, self.y) {
                    execute!(stdout(), MoveTo(self.x, self.y)).ok();
                    print_flush(level.get_char(self.x, self.y));
                    self.x += 1;
                    self.display();
                }
                self.turn(Direction::East);
            }
            KeyCode::Down => {
                if !level.is_solid(self.x, self.y + 1) {
                    execute!(stdout(), MoveTo(self.x, self.y)).ok();
                    print_flush(level.get_char(self.x, self.y));
                    self.y += 1;
                    self.display();
                }
                self.turn(Direction::South);
            }
            KeyCode::Left => {
                if !level.is_solid(self.x - 1, self.y) {
                    execute!(stdout(), MoveTo(self.x, self.y)).ok(); 
                    print_flush(level.get_char(self.x, self.y));
                    self.x -= 1;
                    self.display();
                }
                self.turn(Direction::West);
            }
            _ => { }
        }
    }
    /// 
    /// Handle the player typing a command
    /// 
    pub fn handle_typing(&self, new_char: char) {
        // TODO
    }
}


// Visuals
impl Player {
    pub fn display(&self) {
        execute!(stdout(), MoveTo(self.x, self.y)).ok();
        match self.facing {
            Direction::North => { print_flush("^"); }
            Direction::East => { print_flush(">"); }
            Direction::South => { print_flush("v"); }
            Direction::West => { print_flush("<"); }
        }
    }
}