struct RenderParameters {
    image_size: vec2<u32>,
    tile_y: u32,
    tile_height: u32,
    upper_left: vec2<f32>,
    step: vec2<f32>,
    max_iterations: u32,
}

@group(0) @binding(0)
var<uniform> parameters: RenderParameters;

@group(0) @binding(1)
var<storage, read_write> pixels: array<u32>;

fn escape_time(start_point: vec2<f32>, max_iterations: u32) -> u32 {
    var current_point = start_point;
    var iteration = 0u;

    while iteration < max_iterations {
        if dot(current_point, current_point) > 4.0 {
            break;
        }

        current_point = vec2<f32>(
            current_point.x * current_point.x - current_point.y * current_point.y,
            2.0 * current_point.x * current_point.y,
        ) + start_point;
        iteration += 1u;
    }

    return iteration;
}

fn intensity(iteration: u32, max_iterations: u32) -> u32 {
    if iteration == max_iterations {
        return 0u;
    }

    return iteration * 255u / max_iterations;
}

@compute @workgroup_size(16, 16)
fn render(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x >= parameters.image_size.x || id.y >= parameters.tile_height {
        return;
    }

    let image_y = parameters.tile_y + id.y;
    let start_point = vec2<f32>(
        parameters.upper_left.x + f32(id.x) * parameters.step.x,
        parameters.upper_left.y - f32(image_y) * parameters.step.y,
    );
    let iteration = escape_time(start_point, parameters.max_iterations);
    let pixel_intensity = intensity(iteration, parameters.max_iterations);

    pixels[id.y * parameters.image_size.x + id.x] = pixel_intensity;
}