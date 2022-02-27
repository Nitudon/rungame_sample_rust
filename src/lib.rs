extern crate gdnative;

mod player;
mod rule;

use gdnative::prelude::*;
use player::Player;
use rule::Rule;

const OBSTACLE_DAMAGE : i32 = 1;

fn init(handle: InitHandle) {
    handle.add_class::<Player>();
    handle.add_class::<Rule>();
}

godot_init!(init);