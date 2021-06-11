automod::dir!("src/");
use gdnative::prelude::*;
fn init(handle: InitHandle) {
    handle.add_class::<player::Player>();
    handle.add_class::<mob::Mob>();
    handle.add_class::<main_scene::Main>();
}
godot_init!(init);
