use std::cmp::max;
use std::ops::Mul;
use gdnative::*;
use gdnative::prelude::*;
use gdnative::api::*;
use OBSTACLE_DAMAGE;

const DEFAULT_PLAYER_SPEED : f32 = 3.0;

#[derive(NativeClass, Default)]
#[inherit(RigidBody)]
#[register_with(Self::register_signals)]
pub struct Player {
    #[property(default = 3.0)]
    speed: f32,
    #[property(default = 3.0)]
    jump_power: f32,
    
    life: i32,
    move_velocity: Vector3,

    is_dead: bool,
    is_jumping: bool,
    collision_shape: Option<Ref<CollisionShape>>,
}

#[gdnative::methods]
impl Player {
    fn new(_owner: &RigidBody) -> Self {
        Player{
            speed: DEFAULT_PLAYER_SPEED,
            life: 0,
            ..Default::default()
        }
    }
    
    #[export]
    fn _ready(&mut self, owner: &RigidBody) {
        self.is_dead = false;
        self.is_jumping = false;
        self.move_velocity = Vector3::new(0., 0., self.speed);
        self.collision_shape = Some(unsafe {
            owner
                .get_node_as::<CollisionShape>("CollisionShape")
                .unwrap()
                .claim()
        });
    }

    #[export]
    fn _physics_process(&mut self, owner: &RigidBody, delta: f64) {
        if self.is_dead == false {
            let input = Input::godot_singleton();
            if input.is_action_pressed("ui_left") && self.is_jumping == false {
                self.jump();
            }
            if self.is_jumping && self.move_velocity.y > 0. { 
                self.move_velocity.y -= 0.1;
            }
            if self.move_velocity.y < 0. {
                self.move_velocity.y = 0.;
            }
            
            owner.translate(self.move_velocity.mul(delta as f32));
        }
    }
    
    #[export]
    fn on_hit_obstacle(&mut self, owner: &RigidBody) {
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
        if self.is_jumping || self.is_dead { 
            return;
        }
        
        self.is_jumping = true;
        self.move_velocity.y = 5.0;
    }
    
    fn slide(&mut self) {
        unimplemented!("プレイヤーのスライディング未実装");
    }
}