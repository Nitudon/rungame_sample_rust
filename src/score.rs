use gdnative::*;
use gdnative::prelude::*;
use gdnative::api::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Score {
    pub point: i32,
    pub time: f32,
}

impl Score {
    
}