#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}

// поля температурного расширения стандартного материала
@group(2) @binding(100) var<uniform> temperature: f32;
@group(2) @binding(101) var<uniform> intensity: f32;
@group(2) @binding(103) var<uniform> is_infrared_mode_active: u32;

// фрагментный шейдер материала
@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {

    // получение объекта
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    var out: FragmentOutput;

    // добавление эффекта освещенности
    out.color = apply_pbr_lighting(pbr_input);

    // проверка на то, ключен ли режим отображения в инфракрасном спектре
    if is_infrared_mode_active != 0 {

        // конвертация исходного цвета объекта в серый
        var luminance: f32 = 0.2126 * out.color.r + 0.7152 * out.color.g + 0.0722 * out.color.b;
        var grayscale_color: vec4<f32> = vec4<f32>(luminance, luminance, luminance, out.color.a) * 0.2;

        // смешивание серого с определенной интенсивностью и с учетом температуры
        out.color = mix(grayscale_color, grayscale_color * temperature, intensity);
    }

    return out;
}