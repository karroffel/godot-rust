use gdnative_bindings_generator::*;

use std::env;
use std::fs::File;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut types_output = File::create(out_path.join("bindings_types.rs")).unwrap();
    let mut traits_output = File::create(out_path.join("bindings_traits.rs")).unwrap();
    let mut methods_output = File::create(out_path.join("bindings_methods.rs")).unwrap();

    // gdnative-core already implements all dependencies of Object
    let to_ignore = {
        let visited = std::collections::HashSet::new();
        strongly_connected_components(&Api::new_from_api_json(), "Object", visited)
    };

    generate_bindings(
        &mut types_output,
        &mut traits_output,
        &mut methods_output,
        to_ignore,
    )
    .unwrap();
}
