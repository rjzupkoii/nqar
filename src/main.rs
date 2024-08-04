// main.rs
//
// Main entry point for NQAR.
use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;
use specs_derive::Component;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod systems;
pub use systems::VisibilitySystem;

/// Structure for the state of the game world
pub struct State {
    pub ecs: World
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        draw_map(&self.ecs, ctx);
        
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        // Visibility
        let mut visibility = VisibilitySystem{};
        visibility.run_now(&self.ecs);

        // Maintain step
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
    gs.ecs.register::<Viewshed>();

    // Create the map, start the player in the first room
    let map: Map = Map::new_map();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);
    
    // Create the player entity
    gs.ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Viewshed { visible_tiles: Vec::new(), range: DEFAULT_FOV, dirty: true })
        .build();

    // Run the main loop of th game
    rltk::main_loop(context, gs)
}
