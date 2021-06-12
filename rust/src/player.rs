use gdnative::{
    api::{AnimatedSprite, Area2D, CollisionShape2D, PhysicsBody2D},
    prelude::*,
};
use gdrust::macros::gdrust;

use macros::get_node;

#[gdrust(extends = Area2D)]
#[signal(hit())]
pub struct Player {
    #[export]
    #[default(400)]
    speed: i32,

    screen_size: Vector2,
}

#[methods]
impl Player {
    #[export]
    fn _ready(&mut self, owner: &Area2D) {
        let viewport = owner.get_viewport_rect();
        self.screen_size = viewport.size.to_vector();
        owner.hide();
    }

    #[export]
    fn _process(&mut self, owner: &Area2D, delta: f32) {
        let mut velocity = Vector2::default();
        let input = Input::godot_singleton();

        if Input::is_action_pressed(input, "ui_right") {
            velocity.x += 1.0
        }

        if Input::is_action_pressed(input, "ui_left") {
            velocity.x -= 1.0
        }

        if Input::is_action_pressed(input, "ui_down") {
            velocity.y += 1.0
        }

        if Input::is_action_pressed(input, "ui_up") {
            velocity.y -= 1.0
        }

        let animated_sprite = macros::get_node!(owner, AnimatedSprite, "AnimatedSprite");
        // let sprite = owner.expect_node::<AnimatedSprite, _>("AnimatedSrpite");

        if velocity.length() > 0.0 {
            velocity = velocity.normalize() * self.speed as f32;
            let animation = if velocity.x != 0.0 {
                animated_sprite.set_flip_v(false);
                animated_sprite.set_flip_h(velocity.x < 0.0);
                "walk"
            } else {
                animated_sprite.set_flip_v(velocity.y > 0.0);
                "up"
            };

            animated_sprite.play(animation, false);
        } else {
            animated_sprite.stop()
        }
        let position = owner.global_position() + velocity * delta;
        let position = Vector2::new(
            position.x.max(0.0).min(self.screen_size.x),
            position.y.max(0.0).min(self.screen_size.y),
        );
        owner.set_global_position(position);
    }

    #[export]
    fn on_player_body_entered(&self, owner: &Area2D, _body: Ref<PhysicsBody2D>) {
        owner.hide();
        owner.emit_signal("hit", &[]);

        get_node!(owner, CollisionShape2D, "CollisionShape2D").set_deferred("disabled", true);
    }

    #[export]
    pub fn start(&self, owner: &Area2D, pos: Vector2) {
        owner.set_global_position(pos);
        owner.show();
        get_node!(owner, CollisionShape2D, "CollisionShape2D").set_disabled(false);
    }
}
