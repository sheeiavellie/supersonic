#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

// получение значений из игрового движка
@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    intensity: f32,
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

// фрагментный шейдер
@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // общая степень серости
    let gray_strength: f32 = 0.2 ;

    // сэмплирование
    let color: vec4<f32> = textureSample(screen_texture, texture_sampler, in.uv);

    // конвертация в серый цвет
    let gray: f32 = (0.299 * color.r + 0.587 * color.g + 0.114 * color.b) * gray_strength;
    
    // нормализация серого цвета
    let avg_color: f32 = (color.r + color.g + color.b) / 30.0;
    let result_gray: vec4<f32> = vec4<f32>(avg_color, avg_color, avg_color, color.a);

    // смешивание цветов с учетом интенсивности
    let result: vec4<f32> = mix(color, result_gray, settings.intensity);

    return result;
}