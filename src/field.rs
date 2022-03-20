use gdnative::*;
use gdnative::prelude::*;
use gdnative::api::*;
use Player;

pub trait Field {
    fn on_player_entered(&self, player: &mut Player);
}

#[derive(NativeClass, Default, Debug)]
#[inherit(Area)]
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
    fn _ready(&mut self, owner: &Area) {
        let area = unsafe { owner.get_node(".").unwrap().assume_safe() };
        
        owner
            .connect("body_entered", area, "body_entered", VariantArray::new_shared(), 0)
            .unwrap();
    }

    #[export]
    fn _physics_process(&mut self, owner: &Area, delta: f64) {}

    #[export]
    fn body_entered(&mut self, owner: &Area, data: Variant) {
        let player_node = unsafe {
            data
                .try_to_object::<KinematicBody>()
                .expect("Playerに該当するKinematicBody Nodeが取得できなかった")
                .assume_safe()
        };
        let player = player_node
            .cast_instance::<Player>()
            .expect("Playerに該当するKinematicBodyからPlayer Scriptが取得できなかった");

        player.map_mut(|player, _owner| {
            self.on_player_entered(player);
        }).expect("Player Scriptへのmutableな参照に失敗した");
    }
}

impl Field for AccelerationField {
    fn on_player_entered(&self, player: &mut Player) {
        godot_print!("accel");
        player.accelerate(self.acceleration);
    }
}

#[derive(NativeClass, Default, Debug)]
#[inherit(Area)]
pub struct GoalField;

#[gdnative::methods]
impl GoalField {
    fn new(_owner: &Area) -> Self {
        GoalField
    }

    #[export]
    fn _ready(&mut self, owner: &Area) {
        let area = unsafe { owner.get_node(".").unwrap().assume_safe() };
        
        owner
            .connect("body_entered", area, "body_entered", VariantArray::new_shared(), 0)
            .unwrap();
    }

    #[export]
    fn body_entered(&mut self, owner: &Area, data: Variant) {
        let player_node = unsafe {
            data
                .try_to_object::<KinematicBody>()
                .expect("Playerに該当するKinematicBody Nodeが取得できなかった")
                .assume_safe()
        };
        let player = player_node
            .cast_instance::<Player>()
            .expect("Playerに該当するKinematicBodyからPlayer Scriptが取得できなかった");
        
        player.map_mut(|player, _owner| {
            self.on_player_entered(player);
        }).expect("Player Scriptへのmutableな参照に失敗した");
    }
}

impl Field for GoalField {
    fn on_player_entered(&self, player: &mut Player) {
        godot_print!("goal");
        player.stop();
    }
}