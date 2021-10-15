use crate::prelude::*;

pub fn hud(
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

    // HealthText query (hud parent)

    let (mut transform, size, mut text) = q.q1_mut().single_mut().unwrap();

    text.sections[0].value = format!("Health: {} / {}", player_health.current, player_health.max);

    transform.translation.x = camera_translation.x;
    transform.translation.y = camera_translation.y - proj_bottom;
    transform.translation.z = 999.0;

    let health_height = size.size.height;

    // HealthBar query (hud child)

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

    // InfoText query (hud child)

    let mut transform = q.q3_mut().single_mut().unwrap();
    transform.translation.y = -health_height;
}
