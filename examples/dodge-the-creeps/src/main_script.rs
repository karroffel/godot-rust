use gdnative::*;

use rand::*;

use std;


struct Main {
    header: NativeInstanceHeader,
    score: i32,
}

impl Main {
    pub fn new(header: NativeInstanceHeader) -> Self {
        Main {
            header,
            score: 0,
        }
    }


    pub fn _ready(&mut self) {
        self.new_game();
    }

    pub fn game_over(&mut self) {
        let mut this = Object::cast::<Node2D>(self.as_object()).unwrap();

        let mut score_timer = Object::cast::<Timer>(&this.get_node(NodePath::from_str("ScoreTimer")).unwrap()).unwrap();
        score_timer.stop();

        let mut mob_timer = Object::cast::<Timer>(&this.get_node(NodePath::from_str("MobTimer")).unwrap()).unwrap();
        mob_timer.stop();
    }

    pub fn new_game(&mut self) {
        let mut this = Object::cast::<Node2D>(self.as_object()).unwrap();

        self.score = 0;

        let start_position = Object::cast::<Position2D>(&this.get_node(NodePath::from_str("StartPosition")).unwrap()).unwrap();

        let start_position = start_position.get_position();

        let mut player = this.get_node(NodePath::from_str("Player")).unwrap();

        // hmmmmm, the bindings should provide a way to get the script data
        player.call(GodotString::from_str("start"), &[Variant::from_vector2(&start_position)]);

        let mut start_timer = Object::cast::<Timer>(&this.get_node(NodePath::from_str("StartTimer")).unwrap()).unwrap();
        start_timer.start(-1.0);

    }

    pub fn _on_start_timer_timeout(&mut self) {
        let mut this = Object::cast::<Node2D>(self.as_object()).unwrap();

        let mut score_timer = Object::cast::<Timer>(&this.get_node(NodePath::from_str("ScoreTimer")).unwrap()).unwrap();
        score_timer.start(-1.0);

        let mut mob_timer = Object::cast::<Timer>(&this.get_node(NodePath::from_str("MobTimer")).unwrap()).unwrap();
        mob_timer.start(-1.0);
    }

    pub fn _on_score_timer_timeout(&mut self) {
        self.score += 1;
    }

    pub fn _on_mob_timer_timeout(&mut self) {
        let mut this = Object::cast::<Node2D>(self.as_object()).unwrap();

        let mut mob_spawn_location = Object::cast::<PathFollow2D>(&this.get_node(NodePath::from_str("MobPath/MobSpawnLocation")).unwrap()).unwrap();

        mob_spawn_location.set_unit_offset(thread_rng().gen_range(0.0, 1.0));

        let mob = Object::cast::<PackedScene>(&_ResourceLoader::godot_singleton().load(GodotString::from_str("res://Mob.tscn"), GodotString::from_str(""), true).unwrap()).unwrap();
        let mut mob = Object::cast::<RigidBody2D>(&mob.instance(-1).unwrap()).unwrap();

        let mut direction = mob_spawn_location.get_rotation() + std::f64::consts::PI / 2.0;

        mob.set_position(mob_spawn_location.get_position());

        direction += thread_rng().gen_range(-std::f64::consts::PI / 4.0, std::f64::consts::PI / 4.0);

        mob.set_rotation(direction);


        mob.set_linear_velocity(Vector2::new(300.0, 0.0));

        this.add_child(Object::cast::<Object>(&mob), false);
    }
}

godot_script_class_impls!(Main);


pub fn register_class(handle: init::InitHandle) {
    use gdnative::init::*;

    let class_builder = handle.add_class::<Main>(
        ClassDescriptor {
            name: "Main",
            base_class: "Node2D",
            constructor: Some(godot_wrap_constructor!(Main, Main::new)),
            destructor: Some(godot_wrap_destructor!(Main)),
        }
    );

    class_builder.add_method(
        "_ready",
        godot_wrap_method!(Main, fn _ready(&mut self) -> ())
    );

    class_builder.add_method(
        "game_over",
        godot_wrap_method!(Main, fn game_over(&mut self) -> ())
    );

    class_builder.add_method(
        "new_game",
        godot_wrap_method!(Main, fn new_game(&mut self) -> ())
    );

    class_builder.add_method(
        "_on_mob_timer_timeout",
        godot_wrap_method!(Main, fn _on_mob_timer_timeout(&mut self) -> ())
    );

    class_builder.add_method(
        "_on_score_timer_timeout",
        godot_wrap_method!(Main, fn _on_score_timer_timeout(&mut self) -> ())
    );

    class_builder.add_method(
        "_on_start_timer_timeout",
        godot_wrap_method!(Main, fn _on_start_timer_timeout(&mut self) -> ())
    );
}