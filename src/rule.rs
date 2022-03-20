use gdnative::*;
use gdnative::prelude::*;
use gdnative::api::*;
use Player;

const GAME_START_INTERVAL : f64 = 3.;
const GAME_TIME : f64 = 120.;

#[derive(NativeClass, Default)]
#[inherit(Node)]
pub struct Rule {
    state: GameState,
    point: i32,
    
    start_timer: Option<Ref<Timer, Unique>>,
    game_timer: Option<Ref<Timer, Unique>>,
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

            let game_timer = owner
                .get_node_as::<Timer>("GameTimer")
                .unwrap();

            start_timer
                .connect("timeout", node, "start_game", VariantArray::new_shared(), 0)
                .unwrap();
            self.start_timer = Some(start_timer.claim().assume_unique());
            self.game_timer = Some(game_timer.claim().assume_unique());

            self.ready_game(owner);
        }
    }

    #[export]
    fn _physics_process(&mut self, _owner: &Node, _delta: f64) {
        match self.state {
            GameState::Ready => {}
            GameState::Game => {}
            GameState::Over => {}
        }
    }
    
    #[export]
    fn ready_game(&mut self, owner: &Node) {
        if let Some (start_timer) = self.start_timer.as_ref() {
            start_timer.start(GAME_START_INTERVAL);
        }
        
        godot_print!("game init");
    }

    #[export]
    fn start_game(&mut self, owner: &Node) {
        if let Some (start_timer) = self.start_timer.as_ref() {
            start_timer.stop();
        }
        
        if let Some (game_timer) = self.game_timer.as_ref() {
            game_timer.start(GAME_TIME);
        }

        let player = unsafe {
            owner
                .get_node_as_instance::<Player>("World/Player")
                .expect("Playerに該当するKinematicBodyからPlayer Scriptが取得できなかった")
        };

        player.map_mut(|player, _owner| {
            player.set_active(true);
        }).expect("Player Scriptへのmutableな参照に失敗した");

        godot_print!("game start");
    }

    fn end_game(&mut self) {
        self.state = GameState::Over;
        
        if let Some (game_timer) = self.game_timer.as_ref() {
            game_timer.stop();
        }
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