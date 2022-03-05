use gdnative::*;
use gdnative::prelude::*;
use gdnative::api::*;
use Player;

pub trait Field {
    fn on_player_entered(&self, player: &mut Player);
}

#[derive(NativeClass, Default, Debug)]
#[inherit(Area)]
#[register_with(Self::register_signals)]
pub struct AccelerationField {
    #[property(default = 10.0)]
    acceleration: f32,
}

#[gdnative::methods]
impl AccelerationField {
    fn new(_owner: &Area) -> Self {
        AccelerationField {
            ..Default::default()
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Area) {}

    #[export]
    fn _physics_process(&mut self, owner: &Area, delta: f64) {}

    fn register_signals(builder: &ClassBuilder<Self>) {}
}

impl Field for AccelerationField {
    fn on_player_entered(&self, player: &mut Player) {
        player.accelerate(self.acceleration);
    }
}

#[derive(NativeClass, Default, Debug)]
#[inherit(Area)]
#[register_with(Self::register_signals)]
pub struct GoalField;

#[gdnative::methods]
impl GoalField {
    fn new(_owner: &Area) -> Self {
        GoalField
    }

    #[export]
    fn _ready(&mut self, owner: &Area) {}

    #[export]
    fn _physics_process(&mut self, owner: &Area, delta: f64) {}

    fn register_signals(builder: &ClassBuilder<Self>) {}
}

impl Field for GoalField {
    fn on_player_entered(&self, player: &mut Player) {
        //player.stop();
    }
}