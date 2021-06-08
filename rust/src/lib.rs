mod game;

use gdnative::prelude::{godot_init, godot_print, methods, InitHandle, NativeClass, Node};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct HelloWorld;

// You may add any number of ordinary `impl` blocks as you want. However, ...
impl HelloWorld {
    /// The "constructor" of the class.
    fn new(_owner: &Node) -> Self {
        HelloWorld
    }
}

// Only __one__ `impl` block can have the `#[methods]` attribute, which
// will generate code to automatically bind any exported methods to Godot.
#[methods]
impl HelloWorld {
    // To make a method known to Godot, use the #[export] attribute.
    // In Godot, script "classes" do not actually inherit the parent class.
    // Instead, they are "attached" to the parent object, called the "owner".
    //
    // In order to enable access to the owner, it is passed as the second
    // argument to every single exposed method. As a result, all exposed
    // methods MUST have `owner: &BaseClass` as their second arguments,
    // before all other arguments in the signature.
    #[export]
    fn _ready(&self, _owner: &Node) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        godot_print!("Hello, world!");
    }
}

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<game::Game>();
    handle.add_class::<HelloWorld>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
