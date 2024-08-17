// player.rs
//
// Management of the player.
use std::cmp::{min, max};

use rltk::{Point, Rltk, VirtualKeyCode};
use specs::prelude::*;

use super::{Map, Player, Position, RunState, State, Viewshed};

use crate::map::WINDOW_HEIGHT as WINDOW_HEIGHT;
use crate::map::WINDOW_WIDTH as WINDOW_WIDTH;

/// The default field-of-vision (FOV) for a new player character, in tiles
pub const DEFAULT_FOV: i32 = 8;

/// Try to move the player's character based upon the delta provided
fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        // Get the target location
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        // Don't let the player walk though things
        if map.occupied_tiles[destination_idx] {
            return;
        }

        // Apply the movement to the player
        pos.x = min(WINDOW_WIDTH , max(0, pos.x + delta_x));
        pos.y = min(WINDOW_HEIGHT, max(0, pos.y + delta_y));

        // The everyone know where the player is
        let mut player_pos = ecs.write_resource::<Point>();
        player_pos.x = pos.x;
        player_pos.y = pos.y;

        // Flag the player viewshed as dirty
        viewshed.dirty = true;
    }
}

/// Handle the player input
pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    match ctx.key {
        None => {
            // Nothing happened
            return RunState::Paused
        }
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
    RunState::Running
}