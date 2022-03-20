use gdnative::*;
use gdnative::prelude::*;
use gdnative::api::*;

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
    fn _ready(&mut self, owner: TRef<Node>) {
        /*
        self.start_timer = Some(unsafe {
            owner
                .get_node_as::<Timer>("StartTimer")
                .unwrap()
                .claim()
                .assume_unique()
        });

        self.game_timer = Some(unsafe {
            owner
                .get_node_as::<Timer>("GameTimer")
                .unwrap()
                .claim()
                .assume_unique()
        });*/
    }

    #[export]
    fn _physics_process(&mut self, _owner: &Node, _delta: f64) {
        match self.state {
            GameState::Ready => {}
            GameState::Game => {}
            GameState::Over => {}
        }
    }

    fn start_game(&mut self) {
        if let Some (start_timer) = self.start_timer.as_ref() {
            start_timer.stop();
        }
        
        if let Some (game_timer) = self.game_timer.as_ref() {
            game_timer.start(GAME_TIME);
        }
    }

    #[export]
    fn on_game_timer_ended(&mut self, _owner: &Node) {
        self.end_game();
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