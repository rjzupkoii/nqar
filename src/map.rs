// map.rs
//
// Defines the map for NQAR.
use rltk::{Rltk, RGB};

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
pub fn new_map() -> Vec<TileType> {
    // Allocate the memory for the map
    let mut map = vec![TileType::Floor; (WINDOW_WIDTH + 1) as usize * (WINDOW_HEIGHT + 1) as usize];

    // Make the boundaries walls
    for x in 0..=WINDOW_WIDTH{
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, WINDOW_HEIGHT)] = TileType::Wall;
    }
    for y in 0..=WINDOW_HEIGHT {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(WINDOW_WIDTH, y)] = TileType::Wall;
    }

    map
}

/// Convert from X, Y coordinates to index
pub fn xy_idx(x: i32, y: i32) -> usize{
    (y as usize * (WINDOW_WIDTH + 1) as usize) + x as usize
}