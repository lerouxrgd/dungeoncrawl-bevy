use std::collections::HashSet;
use std::fs::File;

use rand::seq::SliceRandom;
use serde::Deserialize;

use crate::prelude::*;

#[derive(Clone, Deserialize, Debug)]
pub struct Templates {
    pub entities: Vec<Template>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: i32,
    pub name: String,
    pub glyph: char,
    pub sprite_order: usize,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Enemy,
    Item,
}

impl Templates {
    pub fn load() -> Self {
        let file = File::open("assets/template.ron").expect("Failed opening file");
        ron::de::from_reader(file).expect("Unable to load templates")
    }

    pub fn spawn_entities(
        &self,
        level: usize,
        commands: &mut Commands,
        spawn_points: &[Point],
        tilemap: &mut Tilemap,
    ) {
        let mut available_entities = Vec::new();
        self.entities
            .iter()
            .filter(|e| e.levels.contains(&level))
            .for_each(|t| {
                for _ in 0..t.frequency {
                    available_entities.push(t);
                }
            });

        let mut rng = rand::thread_rng();
        spawn_points.iter().for_each(|position| {
            if let Some(entity_spec) = available_entities.as_slice().choose(&mut rng) {
                self.spawn_entity(commands, position, entity_spec, tilemap);
            }
        });
    }

    fn spawn_entity(
        &self,
        commands: &mut Commands,
        position: &Point,
        template: &Template,
        tilemap: &mut Tilemap,
    ) {
        let sprite_index = to_cp437(template.glyph);
        let sprite_order = template.sprite_order;

        let mut entity = commands.spawn();
        entity
            .insert(position.clone())
            .insert(Render {
                sprite_index,
                sprite_order,
            })
            .insert(Name(template.name.clone()));

        match template.entity_type {
            EntityType::Item => {
                entity.insert(Item);
            }
            EntityType::Enemy => {
                entity.insert(Enemy);
                entity.insert(FieldOfView::new(6));
                entity.insert(ChasingPlayer);
                entity.insert(Health {
                    current: template.hp.unwrap(),
                    max: template.hp.unwrap(),
                });
            }
        }

        if let Some(effects) = &template.provides {
            effects
                .iter()
                .for_each(|(provides, n)| match provides.as_str() {
                    "Healing" => {
                        entity.insert(ProvidesHealing { amount: *n });
                    }
                    "MagicMap" => {
                        entity.insert(ProvidesDungeonMap);
                    }
                    _ => {
                        println!("Warning: we don't know how to provide {}", provides);
                    }
                });
        }

        tilemap
            .insert_tile(Tile {
                point: (position.x - CAMERA_OFFSET_X, position.y - CAMERA_OFFSET_Y),
                sprite_index,
                sprite_order,
                tint: Color::WHITE,
            })
            .unwrap();
    }
}
