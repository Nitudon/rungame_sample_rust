use gdnative::*;
use gdnative::prelude::*;
use gdnative::api::*;
use ::{Player, Rule};

// コース上のPlayerに干渉するtrait
pub trait Field {
    fn on_player_entered(&self, player: &mut Player);
}

// 加速するエリア
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
                // Playerの衝突時処理を呼び出す
                player.map_mut(|player, _owner| {
                    self.on_player_entered(player);
                }).expect("Player Scriptへのmutableな参照に失敗した");
            }
        }
    }
}

impl Field for AccelerationField {
    //　ぶつかったらPlayerを加速
    fn on_player_entered(&self, player: &mut Player) {
        player.accelerate(self.acceleration);
    }
}

// 障害物のエリア
#[derive(NativeClass, Default, Debug)]
#[inherit(Area)]
pub struct ObstacleField {
    #[property(default = 10.0)]
    deceleration : f32,
}

#[gdnative::methods]
impl ObstacleField {
    fn new(_owner: &Area) -> Self {
        ObstacleField {
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
    fn body_entered(&mut self, owner: &Area, data: Variant) {
        unsafe {
            // Rigid bodyの取得
            let rigid_body = data.try_to_object::<RigidBody>();
            if rigid_body.is_none() {
                return;
            }

            if let Some(player) = rigid_body.unwrap().assume_safe().cast_instance::<Player>() {
                // Playerの衝突時処理を呼び出して、フィールドを消す
                player.map_mut(|player, _owner| {
                    self.on_player_entered(player);
                }).expect("Player Scriptへのmutableな参照に失敗");
                owner.queue_free();
            }
        }
    }
}

impl Field for ObstacleField {
    // ぶつかったらPlayerを減速
    fn on_player_entered(&self, player: &mut Player) {
        player.decelerate(self.deceleration);
    }
}

// 到達したらゲームを終えるエリア
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
                // Playerの衝突時処理を呼び出す 
                player.map_mut(|player, _owner| {
                    self.on_player_entered(player);
                }).expect("Player Scriptへのmutableな参照に失敗した");

                // Ruleに問い合わせてゲーム終了処理を呼び出す
                let root = owner.get_node_as_instance::<Rule>("/root/Root");
                if let Some(rule) = root {
                    rule.map_mut(|rule, _| {
                        rule.end_game();
                    }).expect("Rule Scriptへのmutableな参照に失敗した"); 
                }
            }
        }
    }
}

impl Field for GoalField {
    // ぶつかったらPlayerを止める
    fn on_player_entered(&self, player: &mut Player) {
        player.stop();
    }
}