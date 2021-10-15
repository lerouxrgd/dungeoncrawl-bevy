use crate::components::Name;
use crate::prelude::*;

pub fn tooltips(
    windows: Res<Windows>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<Camera>>,
    positions_query: Query<(Entity, &Point, &Name, Option<&Health>)>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(pos) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let p = pos - window_size / 2.0;
        let camera_transform = camera_query.single().unwrap();
        let world_pos = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        let map_pos = Point {
            x: (world_pos.x / 32.).floor() as i32 + CAMERA_OFFSET_X,
            y: (world_pos.y / 32.).floor() as i32 + CAMERA_OFFSET_Y,
        };

        for (entity, &pos, name, health) in positions_query.iter() {
            if pos == map_pos {
                let display = if let Some(health) = health {
                    format!("{} : {} hp", &name.0, health.current)
                } else {
                    name.0.clone()
                };

                commands
                    .entity(entity)
                    .insert_bundle(Text2dBundle {
                        text: Text::with_section(
                            display,
                            TextStyle {
                                font: asset_server.load("BigBlue_Terminal_437TT.TTF"),
                                font_size: 10.0,
                                color: Color::WHITE,
                            },
                            TextAlignment {
                                vertical: VerticalAlign::Bottom,
                                horizontal: HorizontalAlign::Right,
                            },
                        ),
                        ..Default::default()
                    })
                    .insert(TooltipText);
            } else {
                commands
                    .entity(entity)
                    .remove_bundle::<Text2dBundle>()
                    .remove::<TooltipText>();
            }
        }
    }
}

pub fn tooltips_display(
    mut tooltips_query: Query<(&mut Transform, &Text2dSize, &Point), With<TooltipText>>,
) {
    for (mut transform, size, pos) in tooltips_query.iter_mut() {
        let tooltip_height = size.size.height;
        transform.translation.x = (pos.x - CAMERA_OFFSET_X) as f32 * 32.;
        transform.translation.y = (pos.y - CAMERA_OFFSET_Y + 1) as f32 * 32. - tooltip_height;
        transform.translation.z = 999.0;
    }
}
