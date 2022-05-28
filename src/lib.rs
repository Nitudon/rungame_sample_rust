extern crate gdnative;

mod player;
mod rule;
mod field;
mod screen;

use gdnative::prelude::*;
use field::{AccelerationField, GoalField};
use player::Player;
use rule::Rule;
use screen::Screen;

const OBSTACLE_DAMAGE : i32 = 1;

fn init(handle: InitHandle) {
    handle.add_class::<Player>();
    handle.add_class::<GoalField>();
    handle.add_class::<AccelerationField>();
    handle.add_class::<Rule>();
    handle.add_class::<Screen>()
}

godot_init!(init);