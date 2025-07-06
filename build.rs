use std::process::Command;
use std::path::Path;

fn main() {
    // Compile GSettings schemas
    let schema_dir = Path::new("data/schemas");
    let status = Command::new("glib-compile-schemas")
        .arg(schema_dir)
        .status()
        .expect("Failed to run glib-compile-schemas");

    if !status.success() {
        panic!("glib-compile-schemas failed with status: {:?}", status);
    }

    // Compile GResource file
    glib_build_tools::compile_resources(
        &["data/resources"],
        "data/resources/resources.gresource.xml",
        "todo_8.gresource",
    );
}
