use bevy::{platform::collections::HashMap, prelude::*};
use bevy_hanabi::prelude::*;
use ron::from_str;
use std::fs::File;
use std::io;
use std::io::Read;

use omagari::controller::ExportedProject;

struct PreparedEffect {
    name: String,
    parent: Option<String>,
    texture_index: Option<usize>,
    effect_handle: Handle<EffectAsset>,
}

struct EffectComplex {
    omagari_project: ExportedProject,
    prepared_effects: Vec<PreparedEffect>,
}

impl EffectComplex {
    pub fn setup(
        filename: &str,
        mut effects: ResMut<Assets<EffectAsset>>,
    ) -> Result<Self, io::Error> {
        let mut file = File::open(filename)?;
        let mut ron_string = String::new();
        file.read_to_string(&mut ron_string)?;
        let omagari_project: ExportedProject =
            from_str(&ron_string).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let mut prepared_effects: Vec<PreparedEffect> = Vec::new();
        for effect in omagari_project.effects.iter() {
            let h = effects.add(effect.effect_asset.clone());
            prepared_effects.push(PreparedEffect {
                name: effect.name.to_string(),
                parent: effect.parent.clone(),
                texture_index: effect.texture_index.clone(),
                effect_handle: h.clone(),
            });
        }
        Ok(Self {
            omagari_project,
            prepared_effects,
        })
    }

    pub fn spawn(&self, commands: &mut Commands, textures: Vec<Handle<Image>>) {
        let mut refs: HashMap<String, Entity> = HashMap::new();
        for prepared_effect in self.prepared_effects.iter() {
            let mut e = commands.spawn((
                Name::new(prepared_effect.name.clone()),
                ParticleEffect::new(prepared_effect.effect_handle.clone()),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ));

            if let Some(texture_index) = prepared_effect.texture_index {
                e.insert(EffectMaterial {
                    images: vec![textures[texture_index].clone()],
                });
            }

            refs.insert(prepared_effect.name.clone(), e.id());

            if let Some(parent) = &prepared_effect.parent {
                if let Some(entity) = refs.get(parent) {
                    e.insert(EffectParent::new(*entity));
                } else {
                    // Error
                }
            }
        }
    }
}
