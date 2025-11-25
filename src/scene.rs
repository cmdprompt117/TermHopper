use std::fs;
use std::io::stdout;
use crossterm::execute; use crossterm::cursor::MoveTo;

use term_hopper::{
    cls, print_flush
};

//
/// Contains general scene data read from a .txt descriptor file
/// 
pub struct Scene {
    // Visual map
    map: String,
    // General info
    pub size: (u16, u16),
    pub player_start: (u16, u16),
    pub data_path: String,
    // Tile data
    pub solid: Vec<char>,
}

// SCENE FILE STRUCTURE:

//
// map data map data map data map data
// map data map data map data map data
// map data map data map data map data
// map data map data map data map data
// map data map data map data map data
// map data map data map data map data
// map data map data map data map data
// INFO
// size x y 
// start x y 
// solid ... ... ... ...
//

impl Scene {
    ///
    /// Load a scene from file
    /// 
    /// TODO replace panics with game error handling system when it exists
    /// 
    pub fn load_from(file_path: String) -> Scene {
        let file_data = fs::read_to_string(&file_path);
        if file_data.is_err() {
            panic!("Could not read Scene from path: {}", file_path);
        }
        let file_data = file_data.unwrap();
        // Validate scene data format
        let scene_info: Vec<&str> = file_data.split("INFO").collect();
        if scene_info.len() != 2 {
            panic!("Invalid scene data format: INFO tag appears multiple times");
        }
        // Get map visuals
        let visual_map = scene_info[0].trim();
        print_flush("map:\n");
        print_flush(visual_map);
        // Get map settings
        let settings: Vec<&str> = scene_info[1].split('\n').collect();
        print_flush("\nsettings:\n");
        print_flush(scene_info[1]);
        let mut size: Option<(u16, u16)> = None;
        let mut start: Option<(u16, u16)> = None;
        let mut solid: Option<Vec<char>> = None;
        for line in settings {
            let split_line: Vec<&str> = line.trim().split(' ').collect();
            match split_line[0] {
                "size" => {
                    let x = split_line[1].parse::<u16>();
                    let y = split_line[2].parse::<u16>();
                    if x.is_err() {
                        panic!("Invalid scene data format: size.0 invalid");
                    }
                    if y.is_err() {
                        panic!("Invalid scene data format: size.1 invalid");
                    }
                    // print_flush(format!("size = {},{}", x.clone().unwrap(), y.clone().unwrap()));
                    size = Some((x.unwrap(), y.unwrap()));
                }
                "start" => {
                    let x = split_line[1].parse::<u16>();
                    let y = split_line[2].parse::<u16>();
                    if x.is_err() {
                        panic!("Invalid scene data format: start.0 invalid");
                    }
                    if y.is_err() {
                        panic!("Invalid scene data format: start.1 invalid");
                    }
                    // print_flush(format!("start = {},{}", x.clone().unwrap(), y.clone().unwrap()));
                    start = Some((x.unwrap(), y.unwrap()));
                }
                "solid" => {
                    let mut chars: Vec<char> = Vec::new();
                    for i in 1..split_line.len() {
                        // TODO this sucks. research how to turn this &str into char better
                        chars.push(split_line[i].chars().into_iter().next().unwrap());
                    }
                    if chars.len() == 0 {
                        panic!("Invalid scene data format: No solid chars defined");
                    }
                    solid = Some(chars);
                }
                _ => {
                    if split_line[0] != " " && split_line[0] != "" && split_line[0] != "\n" {
                        panic!("Invalid scene data setting: {}", split_line[0]);
                    }
                }
            }
        }
        // Make sure all necessary settings were set
        if !size.is_some() || !start.is_some() || !solid.is_some() {
            panic!("Invalid scene data format: Missing one of the necessary settings:\n- size\n- start\n- solid list");
        }
        // Return the new scene
        Scene {
            map: visual_map.to_owned(),
            size: size.unwrap(),
            player_start: start.unwrap(),
            data_path: file_path,
            solid: solid.unwrap()
        }
    }
}

// Map data parsing
impl Scene {
    ///
    /// Determine whether the tile at a requested coord is solid
    /// 
    pub fn is_solid(&self, x: u16, y: u16) -> bool {
        let map_rows: Vec<&str> = self.map.split('\n').collect();
        let map_cols: Vec<char> = map_rows[y as usize].chars().collect();
        for c in self.solid.clone() { // TODO clone inneficient
            if map_cols[x as usize] == c {
                return true;
            }
        }
        return false;
    }
    ///
    /// Get the visual tile at a coord
    /// 
    pub fn get_char(&self, x: u16, y: u16) -> char {
        let map_rows: Vec<&str> = self.map.split('\n').collect();
        let map_cols: Vec<char> = map_rows[y as usize].chars().collect();
        return map_cols[x as usize];
    }
}

// Visualization
impl Scene {
    ///
    /// Prints the Scene's `map` data
    /// 
    pub fn display(&self) {
        cls();
        execute!(stdout(), MoveTo(0, 0)).unwrap(); // TODO unwrap is no no 
        print_flush(self.map.clone());
    }
}