use bevy::{prelude::*, reflect::TypeRegistry, utils::Duration};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .register_type::<ComponentA>()
        .register_type::<ComponentB>()
        .add_startup_system(save_scene_system.exclusive_system())
        .add_startup_system(load_scene_system)
        .add_startup_system(infotext_system)
        .add_system(log_system)
        .run();
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct ComponentA {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ComponentB {
    pub value: String,
    #[reflect(ignore)]
    pub _time_since_startup: Duration,
}

impl FromWorld for ComponentB {
    fn from_world(world: &mut World) -> Self {
        let time = world.resource::<Time>();
        ComponentB {
            _time_since_startup: time.time_since_startup(),
            value: "Default Value".to_string(),
        }
    }
}

fn load_scene_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(DynamicSceneBundle {
        scene: asset_server.load("scenes/load_scene_example.scn.ron"),
        ..default()
    });

    asset_server.watch_for_changes().unwrap();
}

fn log_system(query: Query<(Entity, &ComponentA), Changed<ComponentA>>) {
    for (entity, component_a) in &query {
        info!(" Entity({})", entity.id());
        info!(
            "    ComponentA: {{ x: {}, y: {} }}\n",
            component_a.x, component_a.y
        );
    }
}

fn save_scene_system(world: &mut World) {
    let mut scene_world = World::new();
    let mut component_b = ComponentB::from_world(world);
    component_b.value = "hello".to_string();
    scene_world.spawn().insert_bundle((
        component_b,
        ComponentA { x: 1.0, y: 2.0 },
        Transform::identity(),
    ));
    scene_world
        .spawn()
        .insert_bundle((ComponentA { x: 3.0, y: 4.0 },));
    
    let type_registry = world.resource::<TypeRegistry>();
    let scene = DynamicScene::from_world(&scene_world, type_registry);

    info!("{}", scene.serialize_ron(type_registry).unwrap());

    // TODO: save scene
}

fn infotext_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(
        TextBundle::from_section(
            "Nothing to see in this window! Check the console output!",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 50.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            ..default()
        }),
    );
}