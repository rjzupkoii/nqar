// monsters.ai
//
// Define the basic AI for most (dumb) monsters.
use rltk::{Point};
use specs::prelude::*;

use crate::{Map, Monster, Position, Viewshed};

pub struct MonsterAI { }

impl<'a> System<'a> for MonsterAI {
    type SystemData = ( WriteExpect<'a, Map>,
                        ReadExpect<'a, Point>,
                        WriteStorage<'a, Viewshed>,
                        ReadStorage<'a, Monster>,
                        WriteStorage<'a, Position>);

    fn run(&mut self, data : Self::SystemData) {
        let (mut map, player_pos, mut viewshed, monster, mut position) = data;

        for (viewshed, monster, monster_position) in (&mut viewshed, &monster, &mut position).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                // Note that the AI is running
                println!("{} shouts insults!", monster.name);

                // Find a path from the monster to the player
                let path = rltk::a_star_search(
                    map.xy_idx(monster_position.x, monster_position.y) as i32,
                    map.xy_idx(player_pos.x, player_pos.y) as i32,
                    &mut *map
                );

                // Move in the player's direction by one step
                if path.success && path.steps.len() > 1 {
                    monster_position.x = path.steps[1] as i32 % map.width;
                    monster_position.y = path.steps[1] as i32 / map.width;
                    viewshed.dirty = true;
                }
            }
        }
    }
}