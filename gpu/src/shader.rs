#[allow(unused, non_snake_case, non_camel_case_types, non_upper_case_globals)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/shader_bindings.rs"));
}

pub(super) use bindings::mandelbrot::{
    compute::{create_render_pipeline_embed_source, RENDER_WORKGROUP_SIZE},
    RenderParameters, WgpuBindGroup0, WgpuBindGroup0Entries, WgpuBindGroup0EntriesParams,
};
