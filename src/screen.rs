use gdnative::*;
use gdnative::prelude::*;
use gdnative::api::*;
use rule::GameState;

#[derive(NativeClass, Default)]
#[inherit(Control)]
pub struct Screen {
    // Screen
    start_screen: Option<Ref<Control, Unique>>,
    game_screen: Option<Ref<Control, Unique>>, 
    
    // Start UI
    countdown_label: Option<Ref<Label, Unique>>,
    
    // Game UI
    time_label: Option<Ref<Label, Unique>>,
    speed_label: Option<Ref<Label, Unique>>,
}

#[gdnative::methods]
impl Screen {
    fn new(_owner: &Control) -> Self {
        Screen {
            ..Default::default()
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Control) {
        unsafe {
            let start_screen = owner
                .get_node_as::<Control>("StartScreen")
                .unwrap();

            self.start_screen = Some(start_screen.assume_unique());

            let game_screen = owner
                .get_node_as::<Control>("GameScreen")
                .unwrap();

            self.game_screen = Some(game_screen.assume_unique());

            let countdown_label = owner
                .get_node_as::<Label>("StartScreen/Countdown")
                .unwrap();
            self.countdown_label = Some(countdown_label.assume_unique());

            let time_label = owner
                .get_node_as::<Label>("GameScreen/Time")
                .unwrap();
            self.time_label = Some(time_label.assume_unique());

            let speed_label = owner
                .get_node_as::<Label>("GameScreen/Speed")
                .unwrap();
            self.speed_label = Some(speed_label.assume_unique());
        }
    }

    #[export]
    fn _physics_process(&mut self, _owner: &Control, _delta: f64) {
    }
    
    pub fn set_screen_state(&mut self, state: GameState) {
        let mut start_screen_active = false;
        let mut game_screen_active = false;
        match state {
            GameState::Ready => {
                start_screen_active = true;
                game_screen_active = false;
            }
            GameState::Game => {
                start_screen_active = false;
                game_screen_active = true;
            }
            GameState::Over => {
                start_screen_active = false;
                game_screen_active = false;
            }
        }

        godot_print!("{},{}", start_screen_active, game_screen_active);
        
        self.start_screen.as_ref().unwrap().set_visible(start_screen_active);
        self.game_screen.as_ref().unwrap().set_visible(game_screen_active);
    }
    
    pub fn set_player_speed(&mut self, speed: f64) {
        if let Some(speed_label) = &self.speed_label {
            speed_label.set_text(format!("{:.1}km/h", speed));
        }
    }

    pub fn set_countdown(&mut self, count: i64) {
        if let Some(countdown_label) = &self.countdown_label {
            if count > 0 {
                countdown_label.set_text(format!("{}", count)); 
            } else {
                countdown_label.set_text("GO!");
            }
        }
    }
    
    pub fn set_time(&mut self, time: f64) {
        if let Some(time_label) = &self.time_label {
            time_label.set_text(format!("Time: {:.3}", time));
        } 
    }
}