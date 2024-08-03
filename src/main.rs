// main.rs
//
// Main entry point for NQAR.
use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;
use specs_derive::Component;

mod components;
pub use components::*;
mod player;
pub use player::*;

/// Structure for the state of the game world
pub struct State {
    ecs: World
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
