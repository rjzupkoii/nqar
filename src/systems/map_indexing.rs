// map_indexing.rs
//
// Defines the system for map indexing.
use specs::prelude::*;
use crate::{Map, OccupiesTile, Position};

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = ( WriteExpect<'a, Map>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, OccupiesTile>);

    fn run(&mut self, data : Self::SystemData) {
        let (mut map, position, occupiers) = data;

        map.populate_occupied();
        for (position, _blocks) in (&position, &occupiers).join() {
            let idx = map.xy_idx(position.x, position.y);
            map.occupied_tiles[idx] = true;
        }
    }
}