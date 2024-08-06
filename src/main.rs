// main.rs
//
// Main entry point for NQAR.
use rltk::{GameState, Point, Rltk, RGB};
use specs::prelude::*;

mod ai;
pub use ai::MonsterAI;
mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod systems;
pub use systems::VisibilitySystem;

/// The current state of the world
#[derive(PartialEq, Copy, Clone)]
pub enum RunState { Paused, Running }

/// Structure for the state of the game world
pub struct State {
    pub ecs: World,
    pub run_state: RunState
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        if self.run_state == RunState::Running {
            self.run_systems();
            self.run_state = RunState::Paused
        } else {
            self.run_state = player_input(self, ctx);
        }

        draw_map(&self.ecs, ctx);
        
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            // Only render what the player can actually see
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        // Visibility
        let mut visibility = VisibilitySystem{};
        visibility.run_now(&self.ecs);

        // Monster AI
        let mut mob = MonsterAI{};
        mob.run_now(&self.ecs);        

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
        ecs: World::new(),
        run_state: RunState::Running
    };
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    // Create the map
    let map: Map = Map::new_map();
    
    // Create the RNG
    let mut rng = rltk::RandomNumberGenerator::new();

    // Now add some basic monsters to the map, note that we skip the first room so the player can spawn there
    for (count, room) in map.rooms.iter().skip(1).enumerate() {
        // Determine the monster type
        let glyph : rltk::FontCharType;
        let name : String;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => { glyph = rltk::to_cp437('g'); name = "Goblin".to_string(); }
            _ => { glyph = rltk::to_cp437('o'); name = "Orc".to_string(); }
        }        

        // Add the monster to the center of the room
        let (x, y) = room.center();
        gs.ecs.create_entity()
            .with(Position{ x, y })
            .with(Renderable{
                glyph: glyph,
                fg: RGB::named(rltk::GREEN),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Monster{
                name: format!("{} #{}", &name, count),
            })
            .with(Viewshed{ visible_tiles : Vec::new(), range: DEFAULT_FOV, dirty: true })
            .build();
    }    
    
    // Create the player entity
    let (player_x, player_y) = map.rooms[0].center();
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

    // Register the player location
    gs.ecs.insert(Point::new(player_x, player_y));

    // Register the map
    gs.ecs.insert(map);

    // Run the main loop of th game
    rltk::main_loop(context, gs)
}
