// player.rs
//
// Management of the player.
use std::cmp::{min, max};

use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;

use super::{Player, Position, State};

// Constants for the screen bounds
const WINDOW_WIDTH: i32 = 79;
const WINDOW_HEIGHT: i32 = 49;

/// Try to move the player's character based upon the delta provided
fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for (_player, pos) in (&mut players, &mut positions).join() {
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
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}     // Ignore anything else
        },
    }
}