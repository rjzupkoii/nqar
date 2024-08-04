// visibility.rs
//
// Defines the generic visibility system.
use rltk::{field_of_view, Point};
use specs::prelude::*;

use crate::{Map, Player, Position, Viewshed};

pub struct VisibilitySystem { }

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = ( Entities<'a>,
                        WriteExpect<'a, Map>,
                        WriteStorage<'a, Position>,
                        WriteStorage<'a, Viewshed>, 
                        ReadStorage<'a, Player>);

    fn run(&mut self, data : Self::SystemData) {
        let (entities, mut map, pos, mut viewshed, player) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
            viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height );

            // If this is the player, reveal what they can see
            let entity : Option<&Player> = player.get(ent);
            if let Some(entity) = entity {
                for vis in viewshed.visible_tiles.iter() {
                    let idx = map.xy_idx(vis.x, vis.y);
                    map.revealed_tiles[idx] = true;
                }
            }
        }
    }
}