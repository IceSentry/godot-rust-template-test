use gdnative::{
    api::{AnimatedSprite, RigidBody2D},
    prelude::*,
};
use gdrust::macros::gdrust;
use macros::get_node;

#[gdrust(extends = RigidBody2D)]
pub struct Mob {
    #[export]
    #[default(150)]
    min_speed: i32,

    #[export]
    #[default(250)]
    max_speed: i32,
}

#[methods]
impl Mob {
    #[export]
    fn _ready(&self, owner: &RigidBody2D) {
        let animated_sprite = get_node!(owner, AnimatedSprite, "AnimatedSprite");
        let mob_types = unsafe {
            animated_sprite
                .sprite_frames()
                .expect("sprite_frames not found")
                .assume_unique()
                .get_animation_names()
        };

        let mob = mob_types.get(fastrand::i32(0..mob_types.len()));
        animated_sprite.set_animation(mob);
    }

    #[export]
    fn on_visibility_screen_exited(&self, owner: &RigidBody2D) {
        unsafe {
            owner.assume_unique().queue_free();
        }
    }
}
