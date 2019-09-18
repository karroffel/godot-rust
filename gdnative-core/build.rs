use gdnative_bindings_generator::*;
use std::env;
use std::fs::File;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut types_output = File::create(out_path.join("core_types.rs")).unwrap();
    let mut traits_output = File::create(out_path.join("core_traits.rs")).unwrap();
    let mut methods_output = File::create(out_path.join("core_methods.rs")).unwrap();

    let classes = {
        let visited = std::collections::HashSet::new();
        strongly_connected_components(&Api::new_from_api_json(), "Object", visited)
    };

    for class in classes {
        generate_class(
            &mut types_output,
            &mut traits_output,
            &mut methods_output,
            &class,
        )
        .unwrap();
    }
}
