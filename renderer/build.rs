use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=shaders");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let shader_dir = PathBuf::from("shaders");

    if !shader_dir.exists() {
        return;
    }

    let shaders = [
        "cell.vert",
        "cell.frag",
        "cursor.vert",
        "cursor.frag",
        "selection.vert",
        "selection.frag",
    ];

    for shader in &shaders {
        let input = shader_dir.join(shader);
        if !input.exists() {
            continue;
        }
        let output = out_dir.join(format!("{shader}.spv"));

        let status = Command::new("glslc")
            .arg("--target-env=vulkan1.1")
            .arg("-O")
            .arg("-o")
            .arg(&output)
            .arg(&input)
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("cargo:warning=compiled {shader}");
            }
            Ok(s) => {
                panic!("glslc failed for {shader} with status {s}");
            }
            Err(e) => {
                panic!("glslc not found: {e}. Install with `pkg install shaderc` or similar.");
            }
        }
    }
}
