extern crate gdnative;

mod player;
mod rule;

use gdnative::prelude::*;
use player::Player;
use rule::Rule;

const GOAL_POS_Z : f32 = 100.;
const OBSTACLE_DAMAGE : i32 = 1;
const GAME_TIME : f64 = 120.;
const PLAYER_LIFE : i32 = 3;

fn init(handle: InitHandle) {
    handle.add_class::<Player>();
    handle.add_class::<Rule>();
}

godot_init!(init);