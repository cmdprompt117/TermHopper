use crossterm::execute; use crossterm::cursor::MoveTo;
use crossterm::event::KeyCode;
use std::io::stdout;
use serde::{Serialize, Deserialize};

use term_hopper::print_flush;
use crate::room::{Room, TransitionInfo};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub facing: Direction,
    pub x: u16,
    pub y: u16,
    
    pub is_typing: bool,
}

impl Player {
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
    pub fn move_player(&mut self, room: &Room, input: KeyCode) -> Option<TransitionInfo> {
        match input {
            KeyCode::Up => {
                self.turn(Direction::North);
                // Check collision
                if !room.is_solid(self.x, self.y + 1) {
                    self.y += 1;
                    self.display();
                }
                // Check if we have hit a transition
                if room.is_transition(self.x,self.y) {
                    return Some(room.get_transition_info(self.x, self.y));
                }
            }
            KeyCode::Down => {
                self.turn(Direction::South);
                // Check collision
                if !room.is_solid(self.x, self.y - 1) {
                    self.y -= 1;
                    self.display();
                }
                // Check if we have hit a transition
                if room.is_transition(self.x,self.y) {
                    return Some(room.get_transition_info(self.x, self.y));
                }
            }
            KeyCode::Right => {
                self.turn(Direction::East);
                // Check collision
                if !room.is_solid(self.x + 1, self.y) {
                    self.x += 1;
                    self.display();
                }
                // Check if we have hit a transition
                if room.is_transition(self.x,self.y) {
                    return Some(room.get_transition_info(self.x, self.y));
                }
            }
            KeyCode::Left => {
                self.turn(Direction::West);
                // Check collision
                if !room.is_solid(self.x - 1, self.y) {
                    self.x -= 1;
                    self.display();
                }
                // Check if we have hit a transition
                if room.is_transition(self.x,self.y) {
                    return Some(room.get_transition_info(self.x, self.y));
                }
            }
            _ => { panic!("Unknown keycode being handled by player.move_input() - {}", input); }
        }
        return None;
    }
    /// 
    /// Handle the player typing a command
    /// 
    pub fn handle_typing(&self, _new_char: char) {
        todo!();
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