use gdnative::*;
use gdnative::prelude::*;
use gdnative::api::*;

#[derive(NativeClass, Default, Debug)]
#[inherit(KinematicBody)]
#[register_with(Self::register_signals)]
pub struct Player {
    #[property(default = 2.0)]
    horizontal_speed: f32,
    #[property(default = 3.0)]
    max_forward_speed: f32,
    #[property(default = 5.0)]
    jump_power: f32,
    #[property(default = 5.0)]
    move_acceleration: f32,
    #[property(default = 20.0)]
    fall_acceleration: f32,
    
    move_velocity: Vector3,
    is_dead: bool,
    is_active: bool,
    collision_shape: Option<Ref<CollisionShape>>,
}

#[gdnative::methods]
impl Player {
    fn new(_owner: &KinematicBody) -> Self {
        Player{
            horizontal_speed: 2.0,
            max_forward_speed: 3.0,
            jump_power: 5.0,
            move_acceleration: 5.0,
            fall_acceleration: 20.0,
            ..Default::default()
        }
    }
    
    #[export]
    fn _ready(&mut self, owner: &KinematicBody) {
        self.is_dead = false;
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
    fn _physics_process(&mut self, owner: &KinematicBody, delta: f64) {
        if self.is_dead == false && self.is_active {
            let is_on_floor = owner.is_on_floor();
            let input = Input::godot_singleton();
            if self.move_velocity.z < self.max_forward_speed {
                self.move_velocity.z += self.move_acceleration * delta as f32; 
            } else if self.move_velocity.z > self.max_forward_speed {
                self.move_velocity.z -= self.move_acceleration * delta as f32;
            };
            
            if input.is_action_just_pressed("key_space") && is_on_floor {
                self.jump();
                godot_print!("jump")
            }
            
            if input.is_action_pressed("ui_left") && is_on_floor {
                self.move_velocity.x += self.horizontal_speed;
            }

            if input.is_action_pressed("ui_right") && is_on_floor {
                self.move_velocity.x -= self.horizontal_speed;
            }
            
            self.move_velocity.y -= self.fall_acceleration * (delta as f32);
            self.move_velocity = owner.move_and_slide(self.move_velocity, Vector3::new(0., 1., 0.), false, 5, 3., false);
        }
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
        if self.is_dead {
            return;
        }

        self.move_velocity.z += value;
    }
    
    pub fn jump(&mut self) {
        if self.is_dead { 
            return;
        }
        
        self.move_velocity.y = self.jump_power;
        
        godot_print!("jump")
    }

    pub fn stop(&mut self) {
        self.move_velocity = Vector3::zero();
        self.is_active = false;

        godot_print!("stop player")
    }
}