use bevy::{platform::collections::HashMap, prelude::*};
use bevy_hanabi::prelude::*;
use ron::{de::from_str, ser::PrettyConfig};
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    fs::File,
    io::{self, Read, Write},
    rc::Rc,
};

use omagari::*;

use crate::{effect::*, AppContext};

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct OmagariProject {
    pub effects: Vec<EffectEditor>,
}

#[derive(Resource)]
pub struct EffectResource {
    pub effect_handles: Vec<Handle<EffectAsset>>,
    pub textures: Vec<Handle<Image>>,
    pub context: AppContext,
}

pub fn spawn_particle_effects(
    commands: &mut Commands,
    res: &mut EffectResource,
    clone: Rc<RefCell<&mut OmagariProject>>,
    mut effects: ResMut<Assets<EffectAsset>>,
    curr: Query<Entity, With<ParticleEffect>>,
) {
    for h in res.effect_handles.iter() {
        effects.remove(h);
    }
    for e in curr.iter() {
        commands.entity(e).despawn();
    }
    let mut refs: HashMap<String, Entity> = HashMap::new();
    for effect in clone.borrow().effects.iter() {
        let h = effects.add(effect.produce());
        res.effect_handles.push(h.clone());
        let mut e = commands.spawn((
            ParticleEffect::new(h.clone()),
            EffectMaterial {
                images: vec![res.textures[effect.texture_index().unwrap_or(0)].clone()],
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ));
        refs.insert(effect.name().to_string(), e.id());

        if let Some(parent) = &effect.parent() {
            if let Some(entity) = refs.get(parent) {
                e.insert(EffectParent::new(*entity));
            } else {
                // Error
            }
        }
    }
}

pub fn export_effects_to_files(filename: &str, clone: Rc<RefCell<&mut OmagariProject>>) {
    let base = filename.split('.').next().unwrap();
    let other_filename = format!("{}.hanabi.ron", base);
    let mut to_export = ExportedProject::default();
    for effect in clone.borrow().effects.iter() {
        to_export.effects.push(ExportedEffect {
            name: effect.name().to_string(),
            parent: effect.parent().clone(),
            texture_index: effect.texture_index(),
            effect_asset: effect.produce(),
        });
    }
    let ron_string =
        ron::ser::to_string_pretty(&to_export, PrettyConfig::new().new_line("\n".to_string()))
            .unwrap();
    let mut file = File::create(&other_filename).unwrap();
    file.write_all(ron_string.as_bytes()).unwrap();
}

pub fn projects_list() -> Vec<String> {
    let mut files = Vec::new();
    let entries = std::fs::read_dir(".").unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let filename = entry.file_name();
        if filename.to_string_lossy().ends_with(".omagari.ron") {
            files.push(filename.to_string_lossy().into_owned());
        }
    }
    files
}

pub fn load_project(filename: &str) -> Result<OmagariProject, io::Error> {
    let mut file = File::open(filename)?;
    let mut ron_string = String::new();
    file.read_to_string(&mut ron_string)?;
    let graph: OmagariProject =
        from_str(&ron_string).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(graph)
}

pub fn validate_project_filename(filename: &str) -> bool {
    regex::Regex::new(r".*\.omagari\.ron")
        .unwrap()
        .captures(&filename)
        .is_some()
}
