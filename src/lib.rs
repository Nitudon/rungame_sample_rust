extern crate gdnative;

mod player;
mod rule;
mod field;

use gdnative::prelude::*;
use field::{AccelerationField, GoalField};
use player::Player;
use rule::Rule;

const OBSTACLE_DAMAGE : i32 = 1;

fn init(handle: InitHandle) {
    handle.add_class::<Player>();
    handle.add_class::<GoalField>();
    handle.add_class::<AccelerationField>();
    handle.add_class::<Rule>();
}

godot_init!(init);