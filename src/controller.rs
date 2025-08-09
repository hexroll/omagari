use bevy::{platform::collections::HashMap, prelude::*};
use bevy_hanabi::prelude::*;
use ron::de::from_str;
use std::{
    cell::RefCell,
    fs::File,
    io::{self, Read},
    rc::Rc,
};

use crate::OmagariProject;
use crate::editor_prelude::AppContext;

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

pub fn validate_project_filename<P: AsRef<std::path::Path>>(p: P) -> bool {
    p.as_ref().ends_with("omagari.ron")
}
