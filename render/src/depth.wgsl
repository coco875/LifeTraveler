// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@group(0) @binding(2)
var t_depth: texture_depth_2d;

fn blur(size_blur: i32, tex_coords: vec2<f32>) -> vec4<f32> {

    var color : vec4<f32> = textureSample(t_diffuse, s_diffuse, tex_coords);
    let dimensions = textureDimensions(t_diffuse);

    var count: i32 = 1;

    for (var x = -size_blur; x <= size_blur; x++) {
        for (var y = -size_blur; y <= size_blur; y++) {
            var offset: vec2<f32> = vec2<f32>(f32(x)/f32(dimensions.x), f32(y)/f32(dimensions.y));
            color += textureSample(t_diffuse, s_diffuse, tex_coords + offset);
            count += 1;
        }
    }

    color /= f32(count);
    return color;
}

const min_depth = 0.310;
const max_depth = 0.325;
const depth_size = 0.005;
const intensity = 4.0;
const smooth_size = 0.003;

// Bokeh disc.
// by David Hoskins.
// License Creative Commons Attribution-NonCommercial-ShareAlike 3.0 Unported License.

// The Golden Angle is (3.-sqrt(5.0))*PI radians, which doesn't precompiled for some reason.
// The compiler is a dunce I tells-ya!!
const GOLDEN_ANGLE = 2.3999632;

const ITERATIONS = 150;

// const rot = mat2x2(cos(GOLDEN_ANGLE), sin(GOLDEN_ANGLE), -sin(GOLDEN_ANGLE), cos(GOLDEN_ANGLE));
const rot = mat2x2(0.99912285699459591971590886816511, 0.04187501201141596407945481012521, -0.04187501201141596407945481012521, 0.99912285699459591971590886816511);

//-------------------------------------------------------------------------------------------
fn Bokeh(uv:vec2<f32>, radius:f32) -> vec3<f32>{
	var acc = vec3<f32>(0.0);
    var div = acc;
    var r = 1.0;
    var vangle = vec2<f32>(0.0,radius*.01 / sqrt(f32(ITERATIONS)));
    
	for (var j = 0; j < ITERATIONS; j++) {  
        // the approx increase in the scale of sqrt(0, 1, 2, 3...)
        r += 1. / r;
	    vangle = rot * vangle;
        var col = textureSample(t_diffuse, s_diffuse, uv + (-1.0+r) * vangle).xyz; /// ... Sample the image
        // col = col * col *1.8; // ... Contrast it for better highlights - leave this out elsewhere.
		var bokeh = pow(col, vec3(4.0));
		acc += col * bokeh;
		div += bokeh;
	}
	return acc / div;
}

fn smoothstep(pos: f32, smooth_size: f32, value: f32) -> f32 {
    let x = clamp((value - pos) / smooth_size, 0.0, 1.0);
    return x * x * (3.0 - 2.0 * x);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    let pixel_depth: f32 = textureSample(t_depth, s_diffuse, in.tex_coords);
    let auto_depth: f32 = textureSample(t_depth, s_diffuse, vec2(0.5, 0.5));
    var blur_level = 0.0;

    // if (pixel_depth < min_depth) {
    //     blur_level = (min_depth - pixel_depth);//*intensity;
    // } else if (pixel_depth > max_depth) {
    //     blur_level = (pixel_depth - max_depth);//*intensity);
    // }

    blur_level = smoothstep(auto_depth+depth_size, 0.002, pixel_depth)+(1.0-smoothstep(auto_depth-depth_size, smooth_size, pixel_depth));

    if (pixel_depth == 1.0) {
        blur_level = 0.0;
    }

    // let color: vec4<f32> = vec4<f32>(blur_level, blur_level, blur_level, 1.0);
    // let color: vec4<f32> = blur(i32(blur_level*intensity), in.tex_coords);
    let color = vec4(Bokeh(in.tex_coords, blur_level), 1.0);
    return color;
}