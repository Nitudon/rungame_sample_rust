use std::cmp::max;
use std::ops::Mul;
use gdnative::*;
use gdnative::prelude::*;
use gdnative::api::*;
use OBSTACLE_DAMAGE;

const DEFAULT_PLAYER_SPEED : f32 = 3.0;

#[derive(NativeClass, Default)]
#[inherit(KinematicBody)]
#[register_with(Self::register_signals)]
pub struct Player {
    #[property(default = 3.0)]
    speed: f32,
    #[property(default = 5.0)]
    jump_power: f32,
    #[property(default = 20.0)]
    fall_acceleration: f32,
    
    life: i32,
    move_velocity: Vector3,

    is_dead: bool,
    collision_shape: Option<Ref<CollisionShape>>,
}

#[gdnative::methods]
impl Player {
    fn new(_owner: &KinematicBody) -> Self {
        Player{
            speed: DEFAULT_PLAYER_SPEED,
            life: 0,
            ..Default::default()
        }
    }
    
    #[export]
    fn _ready(&mut self, owner: &KinematicBody) {
        self.is_dead = false;
        self.move_velocity = Vector3::new(0., 0., self.speed);
        self.collision_shape = Some(unsafe {
            owner
                .get_node_as::<CollisionShape>("CollisionShape")
                .unwrap()
                .claim()
        });
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody, delta: f64) {
        if self.is_dead == false {
            let is_on_floor = owner.is_on_floor();
            let input = Input::godot_singleton();
            if input.is_action_just_pressed("key_space") && is_on_floor {
                self.jump();
                godot_print!("jump")
            }
            
            self.move_velocity.z = self.speed;
            self.move_velocity.y -= self.fall_acceleration * (delta as f32);
            self.move_velocity = owner.move_and_slide(self.move_velocity, Vector3::new(0., 1., 0.), false, 5, 3., false);
        }
    }
    
    #[export]
    fn on_hit_obstacle(&mut self, owner: &KinematicBody) {
        self.damage(OBSTACLE_DAMAGE);
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "on_player_dead",
            args: &[],
        });
    }
    
    fn damage(&mut self, damage: i32) {
        if self.is_dead {
            return;
        }
        
        self.life -= damage;
        if self.life <= 0 {
            self.life = 0;
            self.is_dead = true;
        }
    }

    fn stop(&mut self) {
        self.speed = 0.;
    }
    
    fn jump(&mut self) {
        if self.is_dead { 
            return;
        }
        
        self.move_velocity.y = self.jump_power;
    }
    
    fn slide(&mut self) {
        unimplemented!("プレイヤーのスライディング未実装");
    }
}