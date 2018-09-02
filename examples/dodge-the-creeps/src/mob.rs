use super::*;

use gdnative::*;

use rand::*;

const MOB_TYPES: [&'static str; 3] = ["walk", "swim", "fly"];

struct Mob {
    header: NativeInstanceHeader,
    pub min_speed: i32,
    pub max_speed: i32,
}

godot_script_class_impls!(Mob);

impl Mob {

    pub fn new(header: NativeInstanceHeader) -> Self {
        Mob {
            header,
            min_speed: 0,
            max_speed: 0,
        }
    }

    pub fn _ready(&mut self) {
        let this = Object::cast::<RigidBody2D>(self.as_object()).unwrap();

        let animated_sprite = this.get_node(NodePath::from_str("AnimatedSprite")).unwrap();
        let mut animated_sprite = Object::cast::<AnimatedSprite>(&animated_sprite).unwrap();

        let animation_name = MOB_TYPES[rand::thread_rng().gen_range(0, MOB_TYPES.len())];

        animated_sprite.set_animation(GodotString::from_str(animation_name));
    }

    pub fn _on_visibility_screen_exited(&mut self) {
        let mut this = Object::cast::<RigidBody2D>(self.as_object()).unwrap();
        this.queue_free();
    }
}

pub fn register_class(handle: init::InitHandle) {
    use gdnative::init::*;

    let class_builder = handle.add_class::<Mob>(
        ClassDescriptor {
            name: "Mob",
            base_class: "RigidBody2D",
            constructor: Some(godot_wrap_constructor!(Mob, Mob::new)),
            destructor: Some(godot_wrap_destructor!(Mob)),
        }
    );

    class_builder.add_method(
        "_ready",
        godot_wrap_method!(Mob, fn _ready(&mut self) -> ())
    );

    class_builder.add_method(
        "_on_visibility_screen_exited",
        godot_wrap_method!(Mob, fn _on_visibility_screen_exited(&mut self) -> ())
    );

}
