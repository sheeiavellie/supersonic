use bevy::{
    prelude::*, 
    app::{App, Plugin}, 
    pbr::{ExtendedMaterial, MaterialPlugin, StandardMaterial, MaterialExtension}, 
    render::render_resource::{AsBindGroup, ShaderRef}
};

/// Plugin for the materials.
pub struct DefinedMaterialsPlugin;

impl Plugin for DefinedMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MaterialPlugin::<ExtendedMaterial<StandardMaterial, ThermalMaterialExtension>>::default());
    }
}

/// Component that describes whether entity has temperature or not.
#[derive(Component)]
pub struct Thermal;

// materials
/// Thermal `MaterialExtension`.
/// 
/// In order to use it, you need to specify `temperature`, `intensity` and `is_infrared_mode_active`.
/// This values will be sent to a material's fragment shader.
/// 
/// Use `is_infrared_mode_active` to switch into infrared mode and back.
/// 
/// Keep in mind, that infrared white glow only will be applied if `is_infrared_mode_active` is set to 1.
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
pub struct ThermalMaterialExtension {
    #[uniform(100)]
    pub temperature: f32,
    #[uniform(101)]
    pub intensity: f32,
    #[uniform(103)]
    pub is_infrared_mode_active: u32,
}

impl MaterialExtension for ThermalMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/thermal_material.wgsl".into()
    }   
}
