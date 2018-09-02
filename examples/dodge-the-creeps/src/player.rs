use gdnative::*;

use super::*;

struct Player {
    header: NativeInstanceHeader,
    pub speed: i32,
    screen_size: Vector2,
}

godot_script_class_impls!(Player);

impl Player {
    pub fn new(header: NativeInstanceHeader) -> Self {
        Player {
            header,
            speed: 400,
            screen_size: Vector2::new(0.0, 0.0),
        }
    }

    pub fn start(&mut self, pos: Vector2) {
        let mut this = Object::cast::<Area2D>(self.as_object()).unwrap();
        this.set_position(pos);
        this.show();

        let mut collision_shape = Object::cast::<CollisionShape2D>(&this.get_node(NodePath::from_str("CollisionShape2D")).unwrap().as_object()).unwrap();
        collision_shape.set_disabled(false);
    }

    pub fn _ready(&mut self) {
        let mut this = Object::cast::<Area2D>(self.as_object()).unwrap();
        let size = {
            let size = this.get_viewport_rect().size;
            Vector2::new(size.width, size.height)
        };

        self.screen_size = size;

        this.hide();
    }

    pub fn _process(&mut self, delta: f32) {
        let mut this = Object::cast::<Area2D>(self.as_object()).unwrap();

        let input = Input::godot_singleton();

        let mut animated_sprite = {
            let this = Object::cast::<Area2D>(self.as_object()).unwrap();
            let animation_player = this.get_node(NodePath::from_str("AnimatedSprite")).expect("Didn't find AnimatedSprite node");
            Object::cast::<AnimatedSprite>(&animation_player).unwrap()
        };

        let mut velocity = Vector2::new(0.0, 0.0);

        if input.is_action_pressed(GodotString::from_str("ui_right")) {
            velocity.x += 1.0;
        }
        if input.is_action_pressed(GodotString::from_str("ui_left")) {
            velocity.x -= 1.0;
        }
        if input.is_action_pressed(GodotString::from_str("ui_down")) {
            velocity.y += 1.0;
        }
        if input.is_action_pressed(GodotString::from_str("ui_up")) {
            velocity.y -= 1.0;
        }

        if velocity.length() > 0.01 {
            velocity = velocity.normalize() * (self.speed as f32);

            animated_sprite.play(GodotString::from_str(""));
        } else {
            animated_sprite.stop();
        }

        {
            let mut pos = this.get_position();

            pos += velocity * delta;

            pos.x = clamp(pos.x, 0.0, self.screen_size.x);
            pos.y = clamp(pos.y, 0.0, self.screen_size.y);

            this.set_position(pos);
        }

        if velocity.x != 0.0 {
            animated_sprite.set_animation(GodotString::from_str("right"));
            animated_sprite.set_flip_v(false);
            animated_sprite.set_flip_h(velocity.x < 0.0);
        } else {
            animated_sprite.set_animation(GodotString::from_str("up"));
            animated_sprite.set_flip_v(velocity.y > 0.0);
        }
    }

    fn _on_player_body_entered(&mut self, _body: PhysicsBody2D) {
        let mut this = Object::cast::<Area2D>(self.as_object()).unwrap();

        this.hide();
        this.emit_signal(GodotString::from_str("hit"), &[]);

        let mut collision_shape = Object::cast::<CollisionShape2D>(&this.get_node(NodePath::from_str("CollisionShape2D")).unwrap().as_object()).unwrap();
        collision_shape.set_disabled(true);
    }
}

pub fn register_class(handle: init::InitHandle) {
    use gdnative::init::*;

    let constructor = godot_wrap_constructor!(Player, Player::new);
    let destructor = godot_wrap_destructor!(Player);

    let class_builder = handle.add_class::<Player>(
        ClassDescriptor {
            name: Player::class_name(),
            base_class: "Area2D",
            constructor: Some(constructor),
            destructor: Some(destructor),
        }
    );

    class_builder.add_property(godot_create_property!(Player, speed, 400));


    class_builder.add_method(
        "_ready",
        godot_wrap_method!(Player, fn _ready(&mut self) -> ()),
    );
    class_builder.add_method(
        "_process",
        godot_wrap_method!(Player, fn _process(&mut self, delta: f32) -> ()),
    );
    class_builder.add_method(
        "start",
        godot_wrap_method!(Player, fn start(&mut self, pos: Vector2) -> ()),
    );


    class_builder.add_method("_on_player_body_entered", godot_wrap_method!(Player, fn _on_player_body_entered(&mut self, body: PhysicsBody2D) -> ()));

    class_builder.add_signal(Signal {
        name: "hit",
        args: &[],
    });
}