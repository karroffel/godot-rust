#[macro_use]
extern crate gdnative;

extern crate rand;

mod player;
mod mob;
mod main_script;

use gdnative::*;


pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val > max {
        max
    } else if val < min {
        min
    } else {
        val
    }
}


fn nativescript_init(handle: init::InitHandle) {
    player::register_class(handle);
    mob::register_class(handle);
    main_script::register_class(handle);
}

godot_nativescript_init!(nativescript_init);

godot_gdnative_init!();
godot_gdnative_terminate!();
