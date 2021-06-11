use gdnative::prelude::*;

pub fn get_instance<Root>(scene: &Ref<PackedScene, Shared>) -> Ref<Root, Unique>
where
    Root: gdnative::GodotObject<RefKind = ManuallyManaged> + SubClass<Node>,
{
    let scene = unsafe { scene.assume_safe() };

    let instance = scene
        .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
        .expect("failed to instantiate scene");

    let instance = unsafe { instance.assume_unique() };

    instance
        .try_cast::<Root>()
        .expect("failed to cast root node")
}

pub fn fastrand_f64_range(min: f64, max: f64) -> f64 {
    (fastrand::f64() % (max + 1.0 - min)) + min
}

pub fn fastrand_f32_range(min: f32, max: f32) -> f32 {
    (fastrand::f32() % (max + 1.0 - min)) + min
}
