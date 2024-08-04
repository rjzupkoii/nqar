// player.rs
//
// Management of the player.
use std::cmp::{min, max};

use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;

use super::{Player, Position, State, TileType, xy_idx};

use crate::map::WINDOW_HEIGHT as WINDOW_HEIGHT;
use crate::map::WINDOW_WIDTH as WINDOW_WIDTH;

/// Try to move the player's character based upon the delta provided
fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        // Get the target location
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);

        // Don't let the player walk though walls
        if map[destination_idx] == TileType::Wall {
            return;
        }

        // Apply the movement
        pos.x = min(WINDOW_WIDTH , max(0, pos.x + delta_x));
        pos.y = min(WINDOW_HEIGHT, max(0, pos.y + delta_y));
    }
}

/// Handle the player input
pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {}      // Nothing happened
        Some(key) => match key {
            // Up, down, left, right movement
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4 => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right |
            VirtualKeyCode::Numpad6 => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up |
            VirtualKeyCode::Numpad8 => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2 => try_move_player(0, 1, &mut gs.ecs),

            // Diagonal movement
            VirtualKeyCode::Numpad7 => try_move_player(-1, -1, &mut gs.ecs),
            VirtualKeyCode::Numpad9 => try_move_player(1, -1, &mut gs.ecs),
            VirtualKeyCode::Numpad3 => try_move_player(1, 1, &mut gs.ecs),
            VirtualKeyCode::Numpad1 => try_move_player(-1, 1, &mut gs.ecs),

            _ => {}     // Ignore anything else
        },
    }
}