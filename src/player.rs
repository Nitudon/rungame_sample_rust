use gdnative::*;
use gdnative::prelude::*;
use gdnative::api::*;

#[derive(NativeClass, Default, Debug)]
#[inherit(RigidBody)]
#[register_with(Self::register_signals)]
pub struct Player {
    #[property(default = 3.0)]
    max_forward_speed: f32,
    #[property(default = 2.0)]
    move_horizontal_speed: f32,
    #[property(default = 5.0)]
    move_acceleration: f32,
    
    pub move_velocity: Vector3,
    pub is_active: bool,

    mesh_instance: Option<Ref<MeshInstance>>,
}

#[gdnative::methods]
impl Player {
    fn new(_owner: &RigidBody) -> Self {
        Player{
            max_forward_speed: 3.0,
            move_horizontal_speed: 2.0,
            move_acceleration: 5.0,
            ..Default::default()
        }
    }
    
    #[export]
    fn _ready(&mut self, owner: &RigidBody) {
        self.is_active = false;
        self.move_velocity = Vector3::zero();

        self.mesh_instance = Some(unsafe {
            owner
                .get_node_as::<MeshInstance>("Car")
                .unwrap()
                .claim()
        });
    }

    #[export]
    fn _physics_process(&mut self, owner: &RigidBody, delta: f64) {
        // 終了時に飛んできた際の処理
        if self.is_active == false {
            owner.set_linear_velocity(Vector3::zero());
            owner.set_angular_velocity(Vector3::zero());
            return;
        }
        
        // Godotの入力
        let input = Input::godot_singleton();
        
        // 単純な加速処理
        if self.move_velocity.z < self.max_forward_speed {
            self.move_velocity.z += self.move_acceleration * delta as f32; 
        } else if self.move_velocity.z > self.max_forward_speed {
            self.move_velocity.z -= self.move_acceleration * delta as f32;
        };
        
        // 左右の移動速度処理
        self.move_velocity.x = 0.;
        if input.is_action_pressed("ui_left") {
            self.move_velocity.x += self.move_horizontal_speed;
        }

        if input.is_action_pressed("ui_right") {
            self.move_velocity.x -= self.move_horizontal_speed;
        }

        // Rust側の速度をGodot側に反映
        owner.set_linear_velocity(self.move_velocity);
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "stop",
            args: &[],
        });
    }
    
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }
    
    pub fn accelerate(&mut self, value: f32) {
        self.move_velocity.z += value;
    }
    
    pub fn decelerate(&mut self, value: f32) {
        self.move_velocity.z -= value;
        if self.move_velocity.z < 0. {
            self.move_velocity.z = 0.;
        }
    }
    
    pub fn stop(&mut self) {
        self.is_active = false;
    }
}