use std::any::Any;
use gdnative::*;
use gdnative::prelude::*;
use gdnative::api::*;
use ::{Player, Screen};

const GAME_START_INTERVAL : f64 = 3.;
const GAME_TIME : f64 = 120.;

#[derive(NativeClass, Default)]
#[inherit(Node)]
pub struct Rule {
    state: GameState,
    time: f64,
    
    start_timer: Option<Ref<Timer, Unique>>,
}

pub enum GameState {
    Ready,
    Game,
    Over,
}

impl Default for GameState {
    fn default() -> Self { 
        GameState::Ready
    }
}

#[gdnative::methods]
impl Rule {
    fn new(_owner: &Node) -> Self {
        Rule {
            ..Default::default()
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Node) {
        unsafe {
            let node = owner
                .get_node(".")
                .unwrap()
                .assume_safe();

            let start_timer = owner
                .get_node_as::<Timer>("StartTimer")
                .unwrap();

            start_timer
                .connect("timeout", node, "start_game", VariantArray::new_shared(), 0)
                .unwrap();
            self.start_timer = Some(start_timer.claim().assume_unique());
            self.ready_game(owner);
        }
    }

    #[export]
    fn _physics_process(&mut self, owner: &Node, delta: f64) {
        match &self.state {
            GameState::Ready => {
                let screen = unsafe {
                    owner
                        .get_node_as_instance::<Screen>("Screen")
                        .expect("Screenに該当するKinematicBodyからPlayer Scriptが取得できなかった")
                };
                if let Some(start_timer) = &self.start_timer {
                    screen.map_mut(|screen, _| {
                        screen.set_countdown(start_timer.time_left() as i64);
                    }).expect("Player Scriptへのmutableな参照に失敗した");
                }
            }
            GameState::Game => {
                self.time += delta;
                let screen = unsafe {
                    owner
                        .get_node_as_instance::<Screen>("Screen")
                        .expect("Playerに該当するKinematicBodyからPlayer Scriptが取得できなかった")
                };

                let player = unsafe {
                    owner
                        .get_node_as_instance::<Player>("World/Player")
                        .expect("Player Scriptが取得できなかった")
                };
                
                screen.map_mut(|screen, _| {
                    screen.set_time(self.time);
                    player.map(|player, _| {
                        screen.set_player_speed(player.move_velocity.z as f64)
                    }).expect("Player Scriptへのmutableな参照に失敗した");
                }).expect("Screen Scriptへのmutableな参照に失敗した");
            }
            GameState::Over => {}
        }
    }
    
    #[export]
    fn ready_game(&mut self, owner: &Node) {
        if let Some (start_timer) = self.start_timer.as_ref() {
            start_timer.start(GAME_START_INTERVAL);
        }

        self.state = GameState::Ready;
        let screen = unsafe {
            owner
                .get_node_as_instance::<Screen>("Screen")
                .expect("Playerに該当するKinematicBodyからPlayer Scriptが取得できなかった")
        };
        screen.map_mut(|screen, _| {
            screen.set_screen_state(GameState::Ready);
        }).expect("Player Scriptへのmutableな参照に失敗した");
        
        godot_print!("game init");
    }

    #[export]
    fn start_game(&mut self, owner: &Node) {
        if let Some (start_timer) = self.start_timer.as_ref() {
            start_timer.stop();
        }
        
        let player = unsafe {
            owner
                .get_node_as_instance::<Player>("World/Player")
                .expect("Playerに該当するKinematicBodyからPlayer Scriptが取得できなかった")
        };

        player.map_mut(|player, _owner| {
            player.set_active(true);
        }).expect("Player Scriptへのmutableな参照に失敗した");

        self.time = 0.;
        self.state = GameState::Game;
        let screen = unsafe {
            owner
                .get_node_as_instance::<Screen>("Screen")
                .expect("Playerに該当するKinematicBodyからPlayer Scriptが取得できなかった")
        };
        screen.map_mut(|screen, _| {
            screen.set_screen_state(GameState::Game);
        }).expect("Player Scriptへのmutableな参照に失敗した");
        
        self.time = 0.;
        godot_print!("game start");
    }

    pub fn end_game(&mut self) {
        self.state = GameState::Over;
    }

    #[export]
    fn on_player_finished(&mut self, _owner: &Node, data: Variant) {
        if let Some(collision) = data.try_to_object::<KinematicBody>() {
            unsafe {
                let player = collision.assume_safe();
                player.call("stop", &[]); 
            }
        }
    }
}