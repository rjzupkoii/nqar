// map.rs
//
// Defines the map for NQAR.
use std::cmp::{min, max};

use rltk::{Algorithm2D, BaseMap, Point, RandomNumberGenerator, Rltk, RGB};
use specs::prelude::*;

mod rectangle;
pub use rectangle::*;

// Constants for the screen bounds
pub const WINDOW_WIDTH: i32 = 79;
pub const WINDOW_HEIGHT: i32 = 49;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Floor, Wall
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub rooms: Vec<Rectangle>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    /// Generate a new map
    pub fn new_map() -> Map {
        // Allocate the memory for the map and rooms
        let length = (WINDOW_HEIGHT * WINDOW_WIDTH) as usize;
        let mut map = Map {
            tiles: vec![TileType::Wall; length],
            revealed_tiles: vec![false; length],
            visible_tiles: vec![false; length],
            rooms: Vec::new(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT            
        };

        // TODO Placeholder constants for the rooms
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        // Prepare the RNG
        let mut rng = RandomNumberGenerator::new();

        for _idx in 0..MAX_ROOMS {
            // Create a new room
            let width = rng.range(MIN_SIZE, MAX_SIZE);
            let height = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, WINDOW_WIDTH - width - 1) - 1;
            let y = rng.roll_dice(1, WINDOW_HEIGHT - height - 1) - 1;
            let new_room = Rectangle::new(x, y, width, height);

            // Check to see if the room can be placed
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    // Nope, try again
                    continue;
                }
            }

            // The location is valid, add the room and the corridors
            map.apply_room_to_map(&new_room);
            if !map.rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                if rng.range(0,2) == 1 {
                    map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                    map.apply_vertical_tunnel(prev_y, new_y, new_x);
                } else {
                    map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                    map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                }
            }
            map.rooms.push(new_room);
        }

        // Return the map
        map
    }

    fn apply_room_to_map(&mut self, room: &Rectangle) {
        // Tile the floor of the room
        for y in room.upper_y +1 ..= room.lower_y {
            for x in room.upper_x + 1 ..= room.lower_x {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, upper_x: i32, lower_x: i32, y: i32) {
        for x in min(upper_x, lower_x) ..= max(upper_x, lower_x) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < (WINDOW_HEIGHT * WINDOW_WIDTH) as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, upper_y: i32, lower_y: i32, x:i32) {
        for y in min(upper_y, lower_y) ..= max(upper_y, lower_y) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < (WINDOW_HEIGHT * WINDOW_WIDTH) as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn is_exit_valid(&self, x:i32, y:i32) -> bool {
        if x < 1 || x > self.width-1 || y < 1 || y > self.height-1 { 
            return false; 
        }
        let idx = self.xy_idx(x, y);
        self.tiles[idx as usize] != TileType::Wall
    }

    /// Convert from X, Y coordinates to index
    pub fn xy_idx(&self, x: i32, y: i32) -> usize{
        ((y * WINDOW_WIDTH) + x) as usize
    }
}

impl Algorithm2D for Map {
    // Return the dimensions of the map
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    // Return the available exits from the given location
    fn get_available_exits(&self, idx:usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let width = self.width as usize;
    
        // Cardinal directions
        if self.is_exit_valid(x - 1, y) { exits.push((idx - 1, 1.0)) };
        if self.is_exit_valid(x + 1, y) { exits.push((idx + 1, 1.0)) };
        if self.is_exit_valid(x, y - 1) { exits.push((idx - width, 1.0)) };
        if self.is_exit_valid(x, y + 1) { exits.push((idx + width, 1.0)) };
    
        exits
    }

    // Return the Pythagorean distance between two points
    fn get_pathing_distance(&self, idx1:usize, idx2:usize) -> f32 {
        let width = self.width as usize;
        let one = Point::new(idx1 % width, idx1 / width);
        let two = Point::new(idx2 % width, idx2 / width);
        rltk::DistanceAlg::Pythagoras.distance2d(one, two)
    }

    // Return true if the tile is opaque
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}

/// Draw the map to the screen
pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    // Get the map
    let map = ecs.fetch::<Map>();
    
    // Draw the map to the screen
    let mut x = 0;
    let mut y = 0;
    for (idx, tile) in map.tiles.iter().enumerate() {
        // Only render what we have seen
        if map.revealed_tiles[idx] {
            
            // Prepare the glyph and foreground color based upon the tile
            let glyph;
            let mut fg;
            match tile {
                TileType::Floor => {
                    glyph = rltk::to_cp437('.');
                    fg = RGB::from_f32(0.5, 0.5, 0.5);
                }
                TileType::Wall => {
                    glyph = rltk::to_cp437('#');
                    fg = RGB::from_f32(1.0, 0.0, 0.0);
                }            
            }

            // Render the tile if it is visible
            if !map.visible_tiles[idx] { fg = fg.to_greyscale() }
            ctx.set(x, y, fg, RGB::from_f32(0.0, 0.0, 0.0), glyph);
        }

        // Move to next coordinates
        x += 1;
        if x > WINDOW_WIDTH - 1 {
            x = 0;
            y += 1;
        }
    }
}  