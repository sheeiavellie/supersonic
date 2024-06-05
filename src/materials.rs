use bevy::{
    prelude::*, 
    app::{App, Plugin}, 
    pbr::{ExtendedMaterial, MaterialPlugin, StandardMaterial, MaterialExtension}, 
    render::render_resource::{AsBindGroup, ShaderRef}
};

pub struct DefinedMaterialsPlugin;

impl Plugin for DefinedMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MaterialPlugin::<ExtendedMaterial<StandardMaterial, ThermalMaterialExtension>>::default());
    }
}

#[derive(Component)]
pub struct Thermal;

// materials
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
