use crate::prelude::*;

pub fn main_hud(
    windows: Res<Windows>,
    mut commands: Commands,
    health_query: Query<&Health, With<Player>>,
    mut q: QuerySet<(
        Query<(&Transform, &OrthographicProjection), With<Camera>>,
        Query<(&mut Transform, &Text2dSize, &mut Text), (With<Hud>, With<HealthText>)>,
        Query<(&mut Transform, Entity), (With<Hud>, With<HealthBar>)>,
        Query<&mut Transform, (With<Hud>, With<InfoText>)>,
    )>,
) {
    let window = windows.get_primary().unwrap();

    // Camera query

    let (camera_transform, proj) = q.q0().single().unwrap();
    let proj_bottom = proj.bottom;
    let camera_translation = camera_transform.translation;

    // Player health query

    let player_health = health_query.single().unwrap();

    // HealthText query (parent)

    let (mut transform, size, mut text) = q.q1_mut().single_mut().unwrap();

    text.sections[0].value = format!("Health: {} / {}", player_health.current, player_health.max);

    transform.translation.x = camera_translation.x;
    transform.translation.y = camera_translation.y - proj_bottom;
    transform.translation.z = 999.0;

    let health_height = size.size.height;

    // HealthBar query (child)

    let (mut transform, health_bar) = q.q2_mut().single_mut().unwrap();
    transform.translation.x = -window.width() / 2.;
    transform.translation.z = -1.;

    let mut path_builder = lyon_tessellation::path::path::Builder::new();
    let rect = shapes::Rectangle {
        width: window.width() * player_health.current as f32 / player_health.max as f32,
        height: health_height,
        origin: shapes::RectangleOrigin::TopLeft,
    };
    rect.add_geometry(&mut path_builder);

    commands
        .entity(health_bar)
        .remove::<Path>()
        .insert(path_builder.build());

    // InfoText query (child)

    let mut transform = q.q3_mut().single_mut().unwrap();
    transform.translation.y = -health_height;
}

pub fn inventory_hud(
    windows: Res<Windows>,
    player_query: Query<Entity, With<Player>>,
    offset_query: Query<&Text2dSize, (With<Hud>, Or<(With<Parent>, With<Children>)>)>,
    mut q: QuerySet<(
        Query<(&Transform, &OrthographicProjection), With<Camera>>,
        Query<(&mut Transform, &Text2dSize, &mut Visible), (With<Hud>, With<InventoryText>)>,
        Query<(&Carried, &Name, &mut Transform, &Text2dSize, &mut Text), With<Item>>,
    )>,
) {
    let window = windows.get_primary().unwrap();
    let x_offset = (window.width() / 2.) - 16.;

    // Camera query

    let (camera_transform, proj) = q.q0().single().unwrap();
    let proj_bottom = proj.bottom;
    let camera_translation = camera_transform.translation;

    // Offset query

    let mut y_offset = offset_query
        .iter()
        .map(|size| size.size.height)
        .sum::<f32>();

    // Prepare InventoryText

    let (mut transform, size, _) = q.q1_mut().single_mut().unwrap();

    transform.translation.x = camera_translation.x - x_offset;
    transform.translation.y = camera_translation.y - proj_bottom - y_offset;
    transform.translation.z = 999.0;

    y_offset += size.size.height;

    // Display carried items

    let player = player_query.single().unwrap();
    let mut nb_carried = 0;
    q.q2_mut()
        .iter_mut()
        .filter(|(carried, ..)| carried.0 == player)
        .enumerate()
        .for_each(|(i, (_, name, mut transform, size, mut text))| {
            text.sections[0].value = format!("{} : {}", i + 1, name.0);

            transform.translation.x = camera_translation.x - x_offset;
            transform.translation.y = camera_translation.y - proj_bottom - y_offset;
            transform.translation.z = 999.0;

            y_offset += size.size.height;
            nb_carried += 1;
        });

    // Display inventory text

    let (_, _, mut visible) = q.q1_mut().single_mut().unwrap();
    visible.is_visible = nb_carried > 0;
}
