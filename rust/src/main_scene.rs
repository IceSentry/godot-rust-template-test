use gdnative::{
    api::{PathFollow2D, Position2D, RigidBody2D},
    prelude::*,
};
use macros::{get_node, get_node_as_instance};

use crate::{
    player::Player,
    utils::{fastrand_f32_range, fastrand_f64_range, get_instance},
};
use gdrust::macros::gdrust;

#[gdrust(extends = Node)]
pub struct Main {
    #[export]
    #[default(PackedScene::new().into_shared())]
    mob: Ref<PackedScene>,
    score: i32,
}

#[methods]
impl Main {
    #[export]
    fn _ready(&mut self, owner: &Node) {
        self.new_game(owner);
    }

    #[export]
    fn game_over(&self, owner: &Node) {
        godot_print!("game_over");
        get_node!(owner, Timer, "ScoreTimer").stop();
        get_node!(owner, Timer, "MobTimer").stop();
    }

    #[export]
    fn new_game(&mut self, owner: &Node) {
        self.score = 0;
        let player = get_node_as_instance!(owner, Player, "Player");
        let start_position = get_node!(owner, Position2D, "StartPosition");
        player
            .map(|player, owner| player.start(&*owner, start_position.position()))
            .expect("failed to call start() on Player");

        get_node!(owner, Timer, "StartTimer").start(0.0);
    }

    #[export]
    fn _on_start_timer_timeout(&self, owner: &Node) {
        get_node!(owner, Timer, "ScoreTimer").start(0.0);
        get_node!(owner, Timer, "MobTimer").start(0.0);
    }

    #[export]
    fn _on_score_timer_timeout(&mut self, _owner: &Node) {
        self.score += 1;
    }

    #[export]
    fn _on_mob_timer_timeout(&mut self, owner: &Node) {
        let mob_spawn_location = macros::get_node!(owner, PathFollow2D, "MobPath/MobSpawnLocation");
        mob_spawn_location.set_offset(fastrand::u32(std::u32::MIN..std::u32::MAX).into());

        let mob_instance = get_instance::<RigidBody2D>(&self.mob);

        mob_instance.set_position(mob_spawn_location.position());

        let mut direction = mob_spawn_location.rotation() + std::f64::consts::FRAC_PI_2;
        direction += fastrand_f64_range(-std::f64::consts::FRAC_PI_4, std::f64::consts::FRAC_PI_4);
        mob_instance.set_rotation(direction);

        mob_instance.set_linear_velocity(Vector2::new(fastrand_f32_range(150.0, 250.0), 0.0));

        // because ownership rules we can only add it as a child after having done the modifications
        owner.add_child(mob_instance, false);
    }
}
