use bevy::{
    core_pipeline::core_2d::Transparent2d,
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::{Indices, MeshVertexAttribute},
        render_asset::RenderAssets,
        render_phase::{AddRenderCommand, DrawFunctions, RenderPhase, SetItemPipeline},
        render_resource::{
            BlendState, ColorTargetState, ColorWrites, Face, FragmentState, FrontFace,
            MultisampleState, PipelineCache, PolygonMode, PrimitiveState, PrimitiveTopology,
            RenderPipelineDescriptor, SpecializedRenderPipeline, SpecializedRenderPipelines,
            TextureFormat, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
        },
        texture::BevyDefault,
        view::VisibleEntities,
        Extract, RenderApp, RenderStage,
    },
    sprite::{
        DrawMesh2d, Mesh2dHandle, Mesh2dPipeline, Mesh2dPipelineKey, Mesh2dUniform,
        SetMesh2dBindGroup, SetMesh2dViewBindGroup,
    },
    utils::FloatOrd,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ColoredMesh2dPlugin)
        .add_startup_system(star)
        .run();
}

fn star(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut star = Mesh::new(PrimitiveTopology::TriangleList);
    let mut v_pos = vec![[0.0, 0.0, 0.0]];
    for i in 0..10 {
        let a = std::f32::consts::FRAC_PI_2 - i as f32 * std::f32::consts::TAU / 10.0;
        let r = (1 - i % 2) as f32 * 100.0 + 100.0;
        v_pos.push([r * a.cos(), r * a.sin(), 0.0]);
    }
    star.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    let mut v_color: Vec<u32> = vec![Color::BLACK.as_linear_rgba_u32()];
    v_color.extend_from_slice(&[Color::YELLOW.as_linear_rgba_u32(); 10]);
    star.insert_attribute(
        MeshVertexAttribute::new("Vertex_Color", 1, VertexFormat::Unit32),
        v_color,
    );

    let mut indices = vec![0, 1, 10];
    for i in 2..=10 {
        indices.extend_from_slice(&[0, i, i - 1]);
    }
    star.set_indices(Some(Indices::U32(indices)));

    commands.spawn_bundle((
        ColoredMesh2d::default(),
        Mesh2dHandle(meshes.add(star)),
        Transform::default(),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
    ));
    commands
        .spawn_bundle(Camera2dBundle::default());
}

#[derive(Component, Default)]
pub struct ColoredMesh2d;

pub struct ColoredMesh2dPipeline {
    mesh2d_pipeline: Mesh2dPipeline,
}

impl FromWorld for ColoredMesh2dPipeline {
    fn from_world(world: &mut World) -> Self {
        Self {
            mesh2d_pipeline: Mesh2dPipeline::from_world(world),
        }
    }
}