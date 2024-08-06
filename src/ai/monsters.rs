// monsters.ai
//
// Define the basic AI for most (dumb) monsters.
use rltk::{Point};
use specs::prelude::*;

use crate::{Monster, Viewshed};

pub struct MonsterAI { }

impl<'a> System<'a> for MonsterAI {
    type SystemData = ( ReadExpect<'a, Point>,
                        ReadStorage<'a, Viewshed>, 
                        ReadStorage<'a, Monster>);

    fn run(&mut self, data : Self::SystemData) {
        let (player_pos, viewshed, monster) = data;
        for (viewshed, monster) in (&viewshed, &monster).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                println!("{} shouts insults!", monster.name);
            }
        }
    }
}