#[macro_use]
extern crate gdnative as godot;

use godot::*;

struct MyClass {
	class_info: GodotClassInfo,

	elapsed_time: f64,
}

impl MyClass {
	fn new(info: GodotClassInfo) -> Self {
		MyClass {
			class_info: info,
			elapsed_time: 0.0,
		}
	}

	fn _ready(&mut self) {
		godot_print!("Hello World!");
	}

	fn _process(&mut self, delta: f64) {
		self.elapsed_time += delta;
	}

	fn _exit_tree(&mut self) {
		godot_print!("MyClass node was running for {} seconds",
			self.elapsed_time);
	}
}

godot_gdnative_init!(godot_rust_gdnative_init);

godot_gdnative_terminate!(godot_rust_gdnative_terminate);

godot_nativescript_init! {
	godot_rust_nativescript_init,
	|handle| {

		let constructor = godot_create_constructor!(MyClass, MyClass::new);
		let destructor  = godot_create_destructor!(MyClass);

		let ready_method = godot_create_method!(MyClass,
			fn _ready(&mut self) -> ());

		let ready_method = GodotScriptMethod {
			name: "_ready",
			method_ptr: Some(ready_method),
			attributes: GodotScriptMethodAttributes {
				rpc_mode: GodotRpcMode::Disabled
			},
			method_data: std::ptr::null_mut(),
			free_func: None
		};

		let process_method = godot_create_method!(MyClass,
			fn _process(&mut self, delta: f64) -> ());
		
		let exit_tree_method = godot_create_method!(MyClass,
			fn _exit_tree(&mut self) -> ());

		let builder = GodotScriptClassBuilder::new();
		builder
			.set_class_name("Test")
			.set_base_class_name("Node")
			.set_constructor(Some(constructor))
			.set_destructor(Some(destructor))

			.add_method_advanced(ready_method)

			.add_method("_process", process_method)
			.add_method("_exit_tree", exit_tree_method)

			.build(handle);
	}
}