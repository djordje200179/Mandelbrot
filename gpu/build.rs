use std::{
    env,
    error::Error,
    fs::{read_to_string, write},
    path::PathBuf,
};
use wgsl_bindgen::{
    RustWgslTypeMap, WgslBindgenOptionBuilder, WgslShaderSourceType, WgslTypeSerializeStrategy,
};

fn main() -> Result<(), Box<dyn Error>> {
    let output = PathBuf::from(env::var("OUT_DIR")?).join("shader_bindings.rs");

    WgslBindgenOptionBuilder::default()
        .workspace_root("shaders")
        .add_entry_point("shaders/mandelbrot.wgsl")
        .serialization_strategy(WgslTypeSerializeStrategy::Bytemuck)
        .type_map(RustWgslTypeMap)
        .shader_source_type(WgslShaderSourceType::EmbedSource)
        .output(&output)
        .build()?
        .generate()?;

    let bindings = read_to_string(&output)?.replace(
        "#![allow(unused, non_snake_case, non_camel_case_types, non_upper_case_globals)]",
        "",
    );
    write(&output, bindings)?;

    println!("cargo:rerun-if-changed=shaders/mandelbrot.wgsl");
    Ok(())
}
