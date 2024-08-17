// components.rs
//
// Define the components used by NQAR.
use rltk;
use specs::prelude::*;
use specs_derive::Component;

/// Structure for monster entities
#[derive(Component, Debug)]
pub struct Monster {
    pub name: String,
}

/// Structure for objects that occupy tiles
#[derive(Component, Debug)]
pub struct OccupiesTile { }

/// Structure for the player entity
#[derive(Component, Debug)]
pub struct Player { }

/// Structure for an entities location
#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// Structure for entities that can be rendered
#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: rltk::RGB,
    pub bg: rltk::RGB,
}

/// Structure for the viewshed that can be rendered
#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}