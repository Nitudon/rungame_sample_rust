use gdnative::*;
use gdnative::prelude::*;
use gdnative::api::*;

#[derive(NativeClass, Default, Debug)]
#[inherit(RigidBody)]
#[register_with(Self::register_signals)]
pub struct Player {
    #[property(default = 2.0)]
    horizontal_speed: f32,
    #[property(default = 3.0)]
    max_forward_speed: f32,
    #[property(default = 5.0)]
    move_acceleration: f32,
    #[property(default = 20.0)]
    fall_acceleration: f32,
    
    pub move_velocity: Vector3,
    pub is_active: bool,

    collision_shape: Option<Ref<CollisionShape>>,
}

#[gdnative::methods]
impl Player {
    fn new(_owner: &RigidBody) -> Self {
        Player{
            horizontal_speed: 2.0,
            max_forward_speed: 3.0,
            move_acceleration: 5.0,
            fall_acceleration: 20.0,
            ..Default::default()
        }
    }
    
    #[export]
    fn _ready(&mut self, owner: &RigidBody) {
        self.is_active = false;
        self.move_velocity = Vector3::zero();
        self.collision_shape = Some(unsafe {
            owner
                .get_node_as::<CollisionShape>("CollisionShape")
                .unwrap()
                .claim()
        });
    }

    #[export]
    fn _physics_process(&mut self, owner: &RigidBody, delta: f64) {
        if self.is_active == false {
            owner.set_linear_velocity(Vector3::zero());
            return;
        }
        
        let input = Input::godot_singleton();
        if self.move_velocity.z < self.max_forward_speed {
            self.move_velocity.z += self.move_acceleration * delta as f32; 
        } else if self.move_velocity.z > self.max_forward_speed {
            self.move_velocity.z -= self.move_acceleration * delta as f32;
        };
        
        self.move_velocity.x = 0.;
        if input.is_action_pressed("ui_left") {
            self.move_velocity.x = self.horizontal_speed;
        }

        if input.is_action_pressed("ui_right") {
            self.move_velocity.x = -self.horizontal_speed;
        }
        
        self.move_velocity.y = self.fall_acceleration * (delta as f32);
        owner.set_linear_velocity(self.move_velocity);
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "on_player_dead",
            args: &[],
        });

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
    
    pub fn stop(&mut self) {
        self.is_active = false;
        godot_print!("stop player")
    }
}