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
use std::fs::create_dir;
use std::path::PathBuf;
use crate::{effect::*, AppContext};

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct OmagariProject {
    pub effects: Vec<EffectEditor>,
}

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct ExportedEffect {
    pub name: String,
    pub parent: Option<String>,
    pub texture_index: Option<usize>,
    pub effect_asset: EffectAsset,
}

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct ExportedProject {
    pub effects: Vec<ExportedEffect>,
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

pub fn despawn_all_particle_effects(
    ongoing_effects: &Query<Entity, With<ParticleEffect>>,
    commands: &mut Commands
){
    for effect_entity in ongoing_effects{
        commands.entity(effect_entity).try_despawn();
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
    let file_path = Folder::ExportedEffects.full_file_path(other_filename);
    let mut file = File::create(file_path).unwrap();
    file.write_all(ron_string.as_bytes()).unwrap();
}

pub fn projects_list() -> Vec<String> {
    let mut files = Vec::new();
    Folder::SavedEffects.make_folder();
    let saved_effects_path = String::from(Folder::SavedEffects.to_path());
    let entries = std::fs::read_dir(saved_effects_path).unwrap();
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
    let file_path = Folder::SavedEffects.full_file_path(String::from(filename));
    let mut file = File::open(file_path)?;
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


type FolderPath = &'static str;
pub enum Folder{
    SavedEffects,
    ExportedEffects,
}

impl Folder{
    pub fn make_folder(&self) -> FolderPath{
        let folder_path = self.to_path();
        let _ = create_dir(String::from(folder_path));
        folder_path
    }

    pub fn full_file_path(&self, file_name: String) -> PathBuf{
        let folder_path = self.make_folder();
        PathBuf::from(String::from(folder_path)).join(file_name)
    }

    pub fn to_path(&self) -> FolderPath {
        match self{
            Folder::SavedEffects => "saved_effects/",
            Folder::ExportedEffects => "exported_effects/",
        }
    }
}

