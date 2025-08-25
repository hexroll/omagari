use bevy::{asset::AssetLoader, platform::collections::HashMap, prelude::*};
use bevy_hanabi::prelude::*;
use ron::de::from_bytes;
use serde::{Deserialize, Serialize};
use std::{
    io::{self}, time::Duration
};

/// OmagariPlugin is a Bevy plugin for managing and despawning exported Omagari effects from assets.
pub struct OmagariPlugin;

impl Plugin for OmagariPlugin {
    /// Configures the Bevy App to use the OmagariPlugin.
    /// It sets systems for effect despawning and initializes effect assets and loaders.
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_effects_on_timer)
           .init_asset::<EffectComplex>()
           .init_asset_loader::<OmagariAssetLoader>();
    }
}

/// EffectComplex represents a collection of prepared effects in the application.
#[derive(Asset, TypePath, Debug)]
pub struct EffectComplex {
    prepared_effects: Vec<PreparedEffect>,
}

impl EffectComplex {
    /// Spawns effects based on the configuration stored in EffectComplex.
    ///
    /// # Parameters
    /// - `commands`: Mutable reference to Commands used to spawn entities.
    /// - `textures`: Vector of image handles used to apply textures to the effects.
    /// - `pos`: Position to spawn the effects at.
    /// - `despawn_in`: Optional duration for when to despawn the effects.
    pub fn spawn(
        &self,
        commands: &mut Commands,
        textures: &Vec<Handle<Image>>,
        pos: Vec3,
        despawn_in: Option<Duration>,
    ) {
        let mut refs: HashMap<String, Entity> = HashMap::new();
        for prepared_effect in self.prepared_effects.iter() {
            let mut e = commands.spawn((
                Name::new(prepared_effect.name.clone()),
                ParticleEffect::new(prepared_effect.effect_handle.clone()),
                Transform::from_translation(pos),
            ));

            if let Some(texture_index) = prepared_effect.texture_index {
                e.insert(EffectMaterial {
                    images: vec![textures[texture_index].clone()],
                });
            }

            if let Some(despawn_in) = despawn_in {
                e.insert(EffectDespawnTimer(Timer::from_seconds(
                    despawn_in.as_secs_f32(),
                    TimerMode::Once,
                )));
            }

            refs.insert(prepared_effect.name.clone(), e.id());

            if let Some(parent) = &prepared_effect.parent {
                if let Some(entity) = refs.get(parent) {
                    e.insert(EffectParent::new(*entity));
                } else {
                    // TODO: Raise an error
                }
            }
        }
    }

    /// Creates an EffectComplex from raw byte data.
    ///
    /// # Parameters
    /// - `bytes`: Vector of bytes representing the effect configuration.
    /// - `load_context`: Mutable reference to LoadContext for managing assets during loading.
    ///
    /// # Returns
    /// - Result containing either the EffectComplex or an IO error.
    fn from_bytes(bytes: Vec<u8>, load_context: &mut bevy::asset::LoadContext<'_>) -> Result<Self, io::Error> {
        let omagari_project: ExportedProject =
            from_bytes(&bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let mut prepared_effects: Vec<PreparedEffect> = Vec::new();
        for effect in omagari_project.effects.iter() {
            let h = load_context.add_labeled_asset("EffectAsset".to_string(), effect.effect_asset.clone());
            prepared_effects.push(PreparedEffect {
                name: effect.name.to_string(),
                parent: effect.parent.clone(),
                texture_index: effect.texture_index.clone(),
                effect_handle: h.clone(),
            });
        }
        Ok(Self { prepared_effects })
    }
}

/// System function that despawns effects whose timers have expired.
///
/// # Parameters
/// - `commands`: Commands used to remove entities.
/// - `effects_with_timer`: Query for entities with an EffectDespawnTimer component.
/// - `time`: A reference to the current time resource.
fn despawn_effects_on_timer(
    mut commands: Commands,
    mut effects_with_timer: Query<(Entity, &mut EffectDespawnTimer)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in effects_with_timer.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            // TODO: Despawn more gracefully by stopping emissions first
            commands.entity(entity).try_despawn();
        }
    }
}

/// Struct representing an exported effect.
#[derive(Resource, Serialize, Deserialize, Default)]
pub struct ExportedEffect {
    pub name: String,
    pub parent: Option<String>,
    pub texture_index: Option<usize>,
    pub effect_asset: EffectAsset,
}

/// Struct representing an exported project containing multiple effects.
#[derive(Resource, Serialize, Deserialize, Default)]
pub struct ExportedProject {
    pub effects: Vec<ExportedEffect>,
}

/// Struct representing a prepared effect with all its dependencies resolved.
#[derive(Debug)]
struct PreparedEffect {
    name: String,
    parent: Option<String>,
    texture_index: Option<usize>,
    effect_handle: Handle<EffectAsset>,
}

/// Component representing a timer for despawning an effect.
#[derive(Component)]
struct EffectDespawnTimer(Timer);

/// Asset loader for Omagari effects.
#[derive(Default)]
struct OmagariAssetLoader;

impl AssetLoader for OmagariAssetLoader {
    type Asset = EffectComplex;
    type Settings = ();
    type Error = std::io::Error;
    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &(),
        mut load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, std::io::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        Ok(EffectComplex::from_bytes(bytes, &mut load_context)?)
    }
}
