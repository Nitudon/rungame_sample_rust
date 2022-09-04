extern crate gdnative;

mod player;
mod rule;
mod field;
mod screen;

use gdnative::prelude::*;
use field::{AccelerationField, GoalField, ObstacleField};
use player::Player;
use rule::Rule;
use screen::Screen;

fn init(handle: InitHandle) {
    handle.add_class::<Player>();
    handle.add_class::<GoalField>();
    handle.add_class::<AccelerationField>();
    handle.add_class::<ObstacleField>();
    handle.add_class::<Rule>();
}

godot_init!(init);
