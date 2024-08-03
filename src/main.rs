// main.rs
//
// Main entry point for NQAR.
use std::cmp::{min, max};

use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use specs_derive::Component;

// Constants for the screen bounds
const WINDOW_WIDTH: i32 = 79;
const WINDOW_HEIGHT: i32 = 49;

/// Structure for an entities location
#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

/// Structure for entities that can be rendered
#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

/// Structure for the player entity
#[derive(Component, Debug)]
struct Player {}

/// Structure for the state of the game world
struct State {
    ecs: World
}

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
fn player_input(gs: &mut State, ctx: &mut Rltk) {
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

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

fn main() -> rltk::BError {
    // Prepare the game window
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("NQAR")
        .build()?;

    // Prepare the components for the game
    let mut gs = State {
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    // Create the player entity
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();

    // Run the main loop of th game
    rltk::main_loop(context, gs)
}
