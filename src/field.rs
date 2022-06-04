use gdnative::*;
use gdnative::prelude::*;
use gdnative::api::*;
use ::{Player, Rule};

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
    fn _physics_process(&mut self, _owner: &Area, _delta: f64) {}

    #[export]
    fn body_entered(&mut self, _owner: &Area, data: Variant) {
        unsafe {
            let kinematic_body = data.try_to_object::<RigidBody>();
            if kinematic_body.is_none() {
                return;
            }

            if let Some(player) = kinematic_body.unwrap().assume_safe().cast_instance::<Player>() {
                player.map_mut(|player, _owner| {
                    self.on_player_entered(player);
                }).expect("Player Scriptへのmutableな参照に失敗した");
            }
        }
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
        unsafe {
            let rigid_body = data.try_to_object::<RigidBody>();
            if rigid_body.is_none() {
                return;
            }

            if let Some(player) = rigid_body.unwrap().assume_safe().cast_instance::<Player>() {
                player.map_mut(|player, _owner| {
                    self.on_player_entered(player);
                }).expect("Player Scriptへのmutableな参照に失敗した");

                let root = owner.get_node_as_instance::<Rule>("/root/Root");
                if let Some(rule) = root {
                    rule.map_mut(|rule, _| {
                        rule.end_game();
                    }).expect("Player Scriptへのmutableな参照に失敗した"); 
                }
            }
        }
    }
}

impl Field for GoalField {
    fn on_player_entered(&self, player: &mut Player) {
        godot_print!("goal");
        player.stop();
    }
}