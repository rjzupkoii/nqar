// map.rs
//
// Defines the map for NQAR.
use std::cmp::{min, max};

use rltk::{RandomNumberGenerator, Rltk, RGB};

mod rectangle;
pub use rectangle::*;

// Constants for the screen bounds
pub const WINDOW_WIDTH: i32 = 79;
pub const WINDOW_HEIGHT: i32 = 49;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Floor, Wall
}

/// Draw the map to the screen
pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut x = 0;
    let mut y = 0;
    
    for tile in map.iter() {
        // Render the tile based upon the tile type
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0.0, 0.0, 0.0), rltk::to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x, y, RGB::from_f32(1.0, 0.0, 0.0), RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
            }            
        }

        // Move to next coordinates
        x += 1;
        if x > WINDOW_WIDTH {
            x = 0;
            y += 1;
        }
    }
}  

/// Generate a new map
pub fn new_map() -> (Vec<Rectangle>, Vec<TileType>) {
    // Allocate the memory for the map and rooms
    let mut map = vec![TileType::Wall; ((WINDOW_HEIGHT + 1) * (WINDOW_WIDTH + 1)) as usize];
    let mut rooms: Vec<Rectangle> = Vec::new();

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
        let x = rng.roll_dice(1, 80 - width - 1) - 1;
        let y = rng.roll_dice(1, 50 - height - 1) - 1;
        let new_room = Rectangle::new(x, y, width, height);

        // Check to see if the room can be placed
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) {
                // Nope, try again
                continue;
            }
        }

        // The location is valid, add the room and the corridors
        apply_room_to_map(&new_room, &mut map);
        if !rooms.is_empty() {
            let (new_x, new_y) = new_room.center();
            let (prev_x, prev_y) = rooms[rooms.len()-1].center();
            if rng.range(0,2) == 1 {
                apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
            } else {
                apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
            }
        }
        rooms.push(new_room);
    }

    (rooms, map)
}

fn apply_room_to_map(room: &Rectangle, map: &mut [TileType]) {
    // Tile the floor of the room
    for y in room.upper_y +1 ..= room.lower_y {
        for x in room.upper_x + 1 ..= room.lower_x {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], upper_x: i32, lower_x: i32, y: i32) {
    for x in min(upper_x, lower_x) ..= max(upper_x, lower_x) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < ((WINDOW_HEIGHT + 1) * (WINDOW_WIDTH + 1)) as usize {
            map[idx as usize] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], upper_y: i32, lower_y: i32, x:i32) {
    for y in min(upper_y, lower_y) ..= max(upper_y, lower_y) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < ((WINDOW_HEIGHT + 1) * (WINDOW_WIDTH + 1)) as usize {
            map[idx as usize] = TileType::Floor;
        }
    }
}

/// Convert from X, Y coordinates to index
pub fn xy_idx(x: i32, y: i32) -> usize{
    (y as usize * (WINDOW_WIDTH + 1) as usize) + x as usize
}