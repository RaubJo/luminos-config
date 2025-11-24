use std::path::Path;
use luminos_config::{generate, dotenv};

fn main() {
    dotenv().ok();
    let config_dir = Path::new("src/config");
    let out_path = config_dir.join("mod.rs");

    generate(config_dir, &out_path).expect("failed to generate mod.rs");

    // Re-run build.rs if any file in config/ changes
    println!("cargo:rerun-if-changed=config");
    println!("cargo:rerun-if-changed=.env");
}
