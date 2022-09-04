use gdnative::*;
use gdnative::prelude::*;
use rule::GameState;

#[derive(Default)]
pub struct Screen {
    // Screen
    start_screen: Option<Ref<Control, Unique>>,
    game_screen: Option<Ref<Control, Unique>>, 
    end_screen: Option<Ref<Control, Unique>>,

    // Start UI
    countdown_label: Option<Ref<Label, Unique>>,
    
    // Game UI
    time_label: Option<Ref<Label, Unique>>,
    speed_label: Option<Ref<Label, Unique>>,

    // End UI
    clear_time_label: Option<Ref<Label, Unique>>,
}

impl Screen {
    pub fn new(screen_root: &Node) -> Self {
        let mut screen = Screen {
            ..Default::default()
        };
        
        // GodotのUIオブジェクトの参照
        unsafe {
            screen.start_screen = Some(screen_root
                .get_node_as::<Control>("StartScreen")
                .unwrap()
                .assume_unique());

            screen.game_screen = Some(screen_root
                .get_node_as::<Control>("GameScreen")
                .unwrap()
                .assume_unique());

            screen.end_screen = Some(screen_root
                .get_node_as::<Control>("EndScreen")
                .unwrap()
                .assume_unique());

            screen.countdown_label = Some(screen_root
                .get_node_as::<Label>("StartScreen/Countdown")
                .unwrap()
                .assume_unique());

            screen.time_label = Some(screen_root
                .get_node_as::<Label>("GameScreen/Time")
                .unwrap()
                .assume_unique());

            screen.speed_label = Some(screen_root
                .get_node_as::<Label>("GameScreen/Speed")
                .unwrap()
                .assume_unique());

            screen.clear_time_label = Some(screen_root
                .get_node_as::<Label>("EndScreen/Time")
                .unwrap()
                .assume_unique());
        }
        
        screen
    }

    pub fn set_screen_state(&mut self, state: GameState) {
        self.start_screen.as_ref().unwrap().set_visible(state == GameState::Ready);
        self.game_screen.as_ref().unwrap().set_visible(state == GameState::Game);
        self.end_screen.as_ref().unwrap().set_visible(state == GameState::Over);
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

    pub fn set_clear_time(&mut self, time: f64) {
        if let Some(clear_time_label) = &self.clear_time_label {
            clear_time_label.set_text(format!("Lap: {:.3}", time));
        }
    }
}