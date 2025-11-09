struct VertexInput {
    @location(0) position: vec3f,
    @location(1) normal: vec3f,
    @location(2) texcoord: vec2f,
};

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) texcoord: vec2f,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4(model.position, 1.0);
    out.texcoord = model.texcoord;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    let origin = vec3(0.0, 0.0, 0.0);
    let sphere_center = vec3(0.0, 0.0, -5.0);
    let sphere_radius = 1.0;

    let dir = get_ray_dir(1.0, in.texcoord);

    let t = intersect_sphere(origin, dir, sphere_center, sphere_radius);

    if t > 0.0 {
        let hit_point = origin + t * dir;
        let normal = normalize(hit_point - sphere_center);
        let sun_dir = normalize(vec3(1.0, 1.0, 1.0));
        let light = saturate(dot(normal, sun_dir));
        return vec4(light, 0.0, 0.0, 1.0);
    }

    return vec4(0.0, 0.0, 0.0, 1.0);
}

fn get_ray_dir(aspect_ratio: f32, texcoord: vec2f) -> vec3f {
    let fov = 1.5708;
    let tan_half_fov = tan(fov / 2.0);
    let x = (texcoord.x - 1.0) * tan_half_fov * aspect_ratio;
    let y = (texcoord.y - 1.0) * tan_half_fov;
    let z = -1.0;
    return normalize(vec3(x, y, z));
}

fn intersect_sphere(origin: vec3f, dir: vec3f, center: vec3f, radius: f32) -> f32 {
    let oc = origin - center;
    let a = 1.0;
    let b = 2.0 * dot(dir, oc);
    let c = dot(oc, oc) - radius * radius;
    let d = b * b - 4.0 * a * c;

    if d > 0.0 {
        let t = (-b - sqrt(d)) / (2.0 * a);
        if t > 0.0 {
            return t;
        }
    }

    return -1.0;
}
