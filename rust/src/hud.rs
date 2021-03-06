use gdnative::prelude::*;
use gdrust::macros::gdrust;
use macros::get_node;

#[gdrust(extends = CanvasLayer)]
#[signal(start_game())]
pub struct Hud;

#[methods]
impl Hud {
    #[export]
    fn _ready(&mut self, _owner: &CanvasLayer) {}

    #[export]
    pub fn show_message(&self, owner: &CanvasLayer, text: String) {
        let message_label = get_node!(owner, Label, "Message");
        message_label.set_text(text);
        message_label.show();

        get_node!(owner, Timer, "MessageTimer").start(0.0);
    }

    #[export]
    pub fn show_game_over(&self, owner: &CanvasLayer) {
        self.show_message(owner, "Game Over".to_string());
        get_node!(owner, Timer, "GameOverTimer").start(0.0);
    }

    #[export]
    fn on_start_button_pressed(&self, owner: &CanvasLayer) {
        get_node!(owner, Button, "StartButton").hide();
        owner.emit_signal("start_game", &[]);
    }

    #[export]
    fn _on_message_timer_timeout(&self, owner: &CanvasLayer) {
        get_node!(owner, Label, "Message").hide();
    }

    pub fn update_score(&self, owner: &CanvasLayer, score: i32) {
        get_node!(owner, Label, "ScoreLabel").set_text(score.to_string());
    }

    #[export]
    fn _on_game_over_timer_timeout(&self, owner: &CanvasLayer) {
        let message_label = get_node!(owner, Label, "Message");
        message_label.set_text("Dodge the creeps");
        message_label.show();

        get_node!(owner, Timer, "StartButtonTimer").start(0.0);
    }

    #[export]
    fn _on_start_button_timer_timeout(&self, owner: &CanvasLayer) {
        println!("show start button");
        get_node!(owner, Button, "StartButton").show();
    }
}
