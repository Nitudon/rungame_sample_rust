use gdnative::*;
use gdnative::prelude::*;
use ::{Player, Screen};

const GAME_START_INTERVAL : f64 = 3.;

#[derive(NativeClass, Default)]
#[inherit(Node)]
pub struct Rule{
    state: GameState,
    time: f64,
    
    start_timer: Option<Ref<Timer, Unique>>,
    screen: Screen,
}

#[derive(PartialEq)]
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
        // 各種Godotのオブジェクトを参照しに行く
        let node = owner.get_node(".").unwrap();
        unsafe {
            let start_timer = owner
                .get_node_as::<Timer>("StartTimer")
                .unwrap();

            start_timer
                .connect("timeout", node, "start_game", VariantArray::new_shared(), 0)
                .unwrap();
            self.start_timer = Some(start_timer
                .claim()
                .assume_unique());
            
            let screen_node = owner
                .get_node("Screen")
                .unwrap()
                .assume_safe();
            self.screen = Screen::new(&screen_node);
        }

        // ゲームロジックの準備処理
        self.ready_game();
    }

    #[export]
    fn _physics_process(&mut self, owner: &Node, delta: f64) {
        let screen = &mut self.screen;
        // ゲームステートの監視
        match &self.state {
            GameState::Ready => {
                // カウントダウンするUIの更新
                if let Some(start_timer) = &self.start_timer {
                    screen.set_countdown(start_timer.time_left() as i64);
                }
            }
            GameState::Game => {
                // 時間の更新
                self.time += delta;
                screen.set_time(self.time);

                let player = unsafe {
                    owner
                        .get_node_as_instance::<Player>("World/Player")
                        .expect("Playerが取得できなかった")
                };

                player.map(|player, _| {
                    screen.set_player_speed(player.move_velocity.z as f64)
                }).expect("Playerを参照できなかった");
            }
            GameState::Over => {}
        }
    }
    
    fn ready_game(&mut self) {
        self.state = GameState::Ready;
        self.screen.set_screen_state(GameState::Ready);
        
        if let Some (start_timer) = self.start_timer.as_ref() {
            start_timer.start(GAME_START_INTERVAL);
        }
    }

    #[export]
    fn start_game(&mut self, owner: &Node) {
        if let Some (start_timer) = self.start_timer.as_ref() {
            start_timer.stop();
        }
        
        let player = unsafe {
            owner
                .get_node_as_instance::<Player>("World/Player")
                .expect("Playerを取得できなかった")
        };

        player.map_mut(|player, _owner| {
            player.set_active(true);
        }).expect("Playerを参照できなかった");

        self.time = 0.;
        self.state = GameState::Game;
        self.screen.set_screen_state(GameState::Game);
    }

    pub fn end_game(&mut self) {
        self.state = GameState::Over;
        self.screen.set_screen_state(GameState::Over);
        self.screen.set_clear_time(self.time);
    }
}