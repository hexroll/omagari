use bevy::prelude::*;

use bevy_egui::*;
use bevy_hanabi::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::editor_prelude::AppContext;
use crate::helpers::*;
use crate::modifiers::ModifierProducer;
use crate::modifiers::RenderModifierProducer;
use crate::modifiers::*;

fn ui_for_modifiers_list<T, R>(
    app: &mut AppContext,
    ui: &mut egui::Ui,
    mut modifiers: &mut Vec<T>,
    label: &str,
    id: egui::Id,
    add_contents: impl FnOnce(&mut egui::Ui, &mut Vec<T>) -> R,
) where
    T: UiProvider,
{
    let id = id.with(label);
    unique_collapsing(id.value(), label, ui).show(ui, |ui| {
        let n_modifiers = modifiers.len();
        for (index, n) in modifiers.iter_mut().enumerate() {
            let swap = ui
                .horizontal(|ui| {
                    if let Some(list_command) = ui_for_list_item(ui, index, n_modifiers) {
                        return Some(list_command);
                    } else {
                        n.draw_ui(app, ui, index as u64);
                    }
                    None
                })
                .inner;
            if let Some(swap) = swap {
                match swap {
                    ListCommand::Remove(i) => {
                        modifiers.remove(i);
                    }
                    ListCommand::Swap((a, b)) => {
                        modifiers.swap(a, b);
                    }
                }
                break;
            }
        }

        ui.menu_button("+", |ui| add_contents(ui, &mut modifiers));
    });
}

#[derive(Serialize, Deserialize)]
pub enum ModifierEditor {
    SetAttribute(SetAttributeModifierEditor),
    InheritAttribute(InheritAttributeModifierEditor),
    SetPositionCircle(SetPositionCircleModifierEditor),
    SetPositionSphere(SetPositionSphereModifierEditor),
    SetVelocityCircle(SetVelocityCircleModifierEditor),
    SetVelocitySphere(SetVelocitySphereModifierEditor),
    SetVelocityTangent(SetVelocityTangentModifierEditor),
    AccelModifier(AccelModifierEditor),
    LinearDragModifier(LinearDragModifierEditor),
    EmitSpawnEventModifier(EmitSpawnEventModifierEditor),
    ConformToSphereModifier(ConformToSphereModifierEditor),
}

impl ModifierEditor {
    fn produce(&self, writer: &ExprWriter) -> ProducedModifier {
        match self {
            ModifierEditor::SetPositionCircle(n) => {
                ProducedModifier::SetPositionCircle(n.produce(writer))
            }
            ModifierEditor::SetPositionSphere(n) => {
                ProducedModifier::SetPositionSphere(n.produce(writer))
            }
            ModifierEditor::SetVelocityCircle(n) => {
                ProducedModifier::SetVelocityCircle(n.produce(writer))
            }
            ModifierEditor::SetVelocitySphere(n) => {
                ProducedModifier::SetVelocitySphere(n.produce(writer))
            }
            ModifierEditor::SetVelocityTangent(n) => {
                ProducedModifier::SetVelocityTangent(n.produce(writer))
            }
            ModifierEditor::SetAttribute(n) => ProducedModifier::SetAttribute(n.produce(writer)),
            ModifierEditor::InheritAttribute(n) => {
                ProducedModifier::InheritAttribute(n.produce(writer))
            }
            ModifierEditor::AccelModifier(n) => ProducedModifier::AccelModifier(n.produce(writer)),
            ModifierEditor::LinearDragModifier(n) => {
                ProducedModifier::LinearDragModifier(n.produce(writer))
            }
            ModifierEditor::EmitSpawnEventModifier(n) => {
                ProducedModifier::EmitSpawnEventModifier(n.produce(writer))
            }
            ModifierEditor::ConformToSphereModifier(n) => {
                ProducedModifier::ConformToSphere(n.produce(writer))
            }
        }
    }
}

impl UiProvider for ModifierEditor {
    fn draw_ui(&mut self, app: &mut AppContext, ui: &mut egui::Ui, index: u64) {
        match self {
            ModifierEditor::SetPositionCircle(n) => n.draw_ui(app, ui, index),
            ModifierEditor::SetPositionSphere(n) => n.draw_ui(app, ui, index),
            ModifierEditor::SetVelocityCircle(n) => n.draw_ui(app, ui, index),
            ModifierEditor::SetVelocitySphere(n) => n.draw_ui(app, ui, index),
            ModifierEditor::SetVelocityTangent(n) => n.draw_ui(app, ui, index),
            ModifierEditor::SetAttribute(n) => n.draw_ui(app, ui, index),
            ModifierEditor::InheritAttribute(n) => n.draw_ui(app, ui, index),
            ModifierEditor::AccelModifier(n) => n.draw_ui(app, ui, index),
            ModifierEditor::LinearDragModifier(n) => n.draw_ui(app, ui, index),
            ModifierEditor::EmitSpawnEventModifier(n) => n.draw_ui(app, ui, index),
            ModifierEditor::ConformToSphereModifier(n) => n.draw_ui(app, ui, index),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum RenderModifierEditor {
    SizeOverLifetime(SizeOverLifetimeModifierEditor),
    ColorOverLifetime(ColorOverLifetimeModifierEditor),
}

impl UiProvider for RenderModifierEditor {
    fn draw_ui(&mut self, app: &mut AppContext, ui: &mut egui::Ui, index: u64) {
        match self {
            RenderModifierEditor::SizeOverLifetime(n) => n.draw_ui(app, ui, index),
            RenderModifierEditor::ColorOverLifetime(n) => n.draw_ui(app, ui, index),
        }
    }
}

#[derive(Serialize, Deserialize)]
enum ProducedModifier {
    SetVelocityTangent(SetVelocityTangentModifier),
    SetPositionSphere(SetPositionSphereModifier),
    SetPositionCircle(SetPositionCircleModifier),
    SetAttribute(SetAttributeModifier),
    InheritAttribute(InheritAttributeModifier),
    SetVelocityCircle(SetVelocityCircleModifier),
    SetVelocitySphere(SetVelocitySphereModifier),
    AccelModifier(AccelModifier),
    LinearDragModifier(LinearDragModifier),
    EmitSpawnEventModifier(EmitSpawnEventModifier),
    ConformToSphere(ConformToSphereModifier),
}

#[derive(Serialize, Deserialize)]
pub struct EffectEditor {
    name: String,
    parent: Option<String>,
    capacity: u32,
    spawner_settings: SpawnerSettings,
    texture_index: Option<usize>,
    init_modifiers: Vec<ModifierEditor>,
    update_modifiers: Vec<ModifierEditor>,
    render_modifiers: Vec<RenderModifierEditor>,
}

impl UiProvider for EffectEditor {
    fn draw_ui(&mut self, app: &mut AppContext, ui: &mut egui::Ui, index: u64) {
        let id = ui.make_persistent_id(format!("effect {}{}", self.name, index));
        ui.vertical(|ui| {
            egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false)
                .show_header(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.set_max_width(240.0);
                        ui.label("Effect:");
                        ui.text_edit_singleline(&mut self.name);
                    });
                })
                .body(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Capacity:");
                        self.capacity = ui_for_u32_ex(ui, self.capacity, 0, 16384, 1);
                        ui.label("Texture:");
                        let mut curr = self.texture_index.unwrap_or(0);
                        let options = PARTICLE_TEXTURES
                            .iter()
                            .map(|v| v.ui_label)
                            .collect::<Vec<&str>>();
                        egui::ComboBox::from_id_salt(99)
                            .selected_text(options[curr])
                            .show_ui(ui, |ui| {
                                for (i, o) in options.iter().enumerate() {
                                    ui.selectable_value(&mut curr, i, *o);
                                }
                            });

                        self.texture_index = Some(curr);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Parent Effect:");
                        let parent = self.parent.as_ref().unwrap_or(&"NONE".to_string()).clone();
                        ui.menu_button(parent, |ui| {
                            for effect in app.visible_effects.iter() {
                                if *effect != self.name {
                                    if ui.button(effect).clicked() {
                                        self.parent = Some(effect.clone());
                                        ui.close_menu();
                                    }
                                }
                            }
                        })
                    });

                    unique_collapsing(1, "Spawner", ui).show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let rate: [f32; 2] = self.spawner_settings.count().range();
                            ui.label("Count:");
                            let rate = ui_for_f32_ex(ui, rate[0], 0.0, 10000.0, 1.0);
                            self.spawner_settings
                                .set_count(CpuValue::Single(rate.into()));
                        });
                        ui.horizontal(|ui| {
                            let duration: [f32; 2] = self.spawner_settings.spawn_duration().range();
                            ui.label("Duration:");
                            let duration = ui_for_f32_ex(ui, duration[0], 0.0, 10000.0, 1.0);
                            self.spawner_settings
                                .set_spawn_duration(CpuValue::Single(duration.into()));
                        });
                        ui.horizontal(|ui| {
                            let period: [f32; 2] = self.spawner_settings.period().range();
                            ui.label("Period:");
                            let period = ui_for_f32_ex(ui, period[0], 0.0, 10000.0, 1.0);
                            self.spawner_settings
                                .set_period(CpuValue::Single(period.into()));
                        });

                        ui.horizontal(|ui| {
                            let cycle_count: u32 = self.spawner_settings.cycle_count();
                            ui.label("Cycles:");
                            let cycle_count = ui_for_u32_ex(ui, cycle_count, 0, 10000, 1);
                            self.spawner_settings.set_cycle_count(cycle_count);
                        });
                    });

                    ui_for_modifiers_list(
                        app,
                        ui,
                        &mut self.init_modifiers,
                        "Init",
                        id,
                        |ui, list| {
                            if ui.button(SetAttributeModifierEditor::label()).clicked() {
                                list.push(ModifierEditor::SetAttribute(
                                    SetAttributeModifierEditor::default(),
                                ));
                            }
                            if ui
                                .button(SetPositionCircleModifierEditor::label())
                                .clicked()
                            {
                                list.push(ModifierEditor::SetPositionCircle(
                                    SetPositionCircleModifierEditor::default(),
                                ));
                            }
                            if ui
                                .button(SetPositionSphereModifierEditor::label())
                                .clicked()
                            {
                                list.push(ModifierEditor::SetPositionSphere(
                                    SetPositionSphereModifierEditor::default(),
                                ));
                            }
                            if ui
                                .button(SetVelocityCircleModifierEditor::label())
                                .clicked()
                            {
                                list.push(ModifierEditor::SetVelocityCircle(
                                    SetVelocityCircleModifierEditor::default(),
                                ));
                            }
                            if ui
                                .button(SetVelocitySphereModifierEditor::label())
                                .clicked()
                            {
                                list.push(ModifierEditor::SetVelocitySphere(
                                    SetVelocitySphereModifierEditor::default(),
                                ));
                            }
                            if ui
                                .button(SetVelocityTangentModifierEditor::label())
                                .clicked()
                            {
                                list.push(ModifierEditor::SetVelocityTangent(
                                    SetVelocityTangentModifierEditor::default(),
                                ));
                            }
                            if ui.button(InheritAttributeModifierEditor::label()).clicked() {
                                list.push(ModifierEditor::InheritAttribute(
                                    InheritAttributeModifierEditor::default(),
                                ));
                            }
                        },
                    );
                    ui_for_modifiers_list(
                        app,
                        ui,
                        &mut self.update_modifiers,
                        "Update",
                        id,
                        |ui, list| {
                            if ui.button(AccelModifierEditor::label()).clicked() {
                                list.push(ModifierEditor::AccelModifier(
                                    AccelModifierEditor::default(),
                                ));
                            }
                            if ui.button(LinearDragModifierEditor::label()).clicked() {
                                list.push(ModifierEditor::LinearDragModifier(
                                    LinearDragModifierEditor::default(),
                                ));
                            }
                            if ui.button(EmitSpawnEventModifierEditor::label()).clicked() {
                                list.push(ModifierEditor::EmitSpawnEventModifier(
                                    EmitSpawnEventModifierEditor::default(),
                                ));
                            }
                            if ui.button(ConformToSphereModifierEditor::label()).clicked() {
                                list.push(ModifierEditor::ConformToSphereModifier(
                                    ConformToSphereModifierEditor::default(),
                                ));
                            }
                        },
                    );
                    ui_for_modifiers_list(
                        app,
                        ui,
                        &mut self.render_modifiers,
                        "Render",
                        id,
                        |ui, list| {
                            if ui.button(SizeOverLifetimeModifierEditor::label()).clicked() {
                                list.push(RenderModifierEditor::SizeOverLifetime(
                                    SizeOverLifetimeModifierEditor::default(),
                                ));
                            }
                            if ui
                                .button(ColorOverLifetimeModifierEditor::label())
                                .clicked()
                            {
                                list.push(RenderModifierEditor::ColorOverLifetime(
                                    ColorOverLifetimeModifierEditor::default(),
                                ));
                            }
                        },
                    );
                });
        });
    }
}

impl EffectEditor {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn parent(&self) -> Option<String> {
        self.parent.clone()
    }
    pub fn texture_index(&self) -> Option<usize> {
        self.texture_index
    }
    pub fn produce(&self) -> EffectAsset {
        let writer = ExprWriter::new();

        let mut init_modifiers: Vec<ProducedModifier> = Vec::new();
        let mut update_modifiers: Vec<ProducedModifier> = Vec::new();
        for m in self.init_modifiers.iter() {
            init_modifiers.push(m.produce(&writer));
        }
        for m in self.update_modifiers.iter() {
            update_modifiers.push(m.produce(&writer));
        }

        let texture_slot = writer.lit(0u32).expr();

        let mut module = writer.finish();
        module.add_texture_slot("color");

        let mut e = EffectAsset::new(self.capacity, self.spawner_settings, module)
            .with_alpha_mode(bevy_hanabi::AlphaMode::Blend)
            .with_name(&self.name);

        for modifier_wrapper in init_modifiers {
            match modifier_wrapper {
                ProducedModifier::SetVelocityTangent(modifier) => {
                    e = e.init(modifier);
                }
                ProducedModifier::SetPositionCircle(modifier) => {
                    e = e.init(modifier);
                }
                ProducedModifier::SetPositionSphere(modifier) => {
                    e = e.init(modifier);
                }
                ProducedModifier::SetAttribute(modifier) => {
                    e = e.init(modifier);
                }
                ProducedModifier::InheritAttribute(modifier) => {
                    e = e.init(modifier);
                }
                ProducedModifier::SetVelocityCircle(modifier) => {
                    e = e.init(modifier);
                }
                ProducedModifier::SetVelocitySphere(modifier) => {
                    e = e.init(modifier);
                }
                _ => {}
            }
        }

        for modifier_wrapper in update_modifiers {
            match modifier_wrapper {
                ProducedModifier::AccelModifier(modifier) => {
                    e = e.update(modifier);
                }
                ProducedModifier::LinearDragModifier(modifier) => {
                    e = e.update(modifier);
                }
                ProducedModifier::EmitSpawnEventModifier(modifier) => {
                    e = e.update(modifier);
                }
                ProducedModifier::ConformToSphere(modifier) => {
                    e = e.update(modifier);
                }
                _ => {}
            }
        }

        for editor_wrapper in self.render_modifiers.iter() {
            match editor_wrapper {
                RenderModifierEditor::SizeOverLifetime(editor) => e = e.render(editor.produce()),
                RenderModifierEditor::ColorOverLifetime(editor) => e = e.render(editor.produce()),
            }
        }

        e.render(ParticleTextureModifier {
            texture_slot,
            sample_mapping: ImageSampleMapping::ModulateOpacityFromR,
        })
        .render(OrientModifier::new(OrientMode::AlongVelocity))
    }
}

impl Default for EffectEditor {
    fn default() -> Self {
        EffectEditor {
            name: "Name your effect".to_string(),
            parent: None,
            capacity: 16384,
            spawner_settings: SpawnerSettings::rate(500.0.into()),
            texture_index: Some(0),
            init_modifiers: Vec::new(),
            update_modifiers: Vec::new(),
            render_modifiers: Vec::new(),
        }
    }
}

pub struct ParticleTexture {
    pub filename: &'static str,
    pub ui_label: &'static str,
}

pub const PARTICLE_TEXTURES: [ParticleTexture; 7] = [
    ParticleTexture {
        filename: "cloud.png",
        ui_label: "Cloud1",
    },
    ParticleTexture {
        filename: "cloud2.png",
        ui_label: "Cloud2",
    },
    ParticleTexture {
        filename: "spark1.png",
        ui_label: "Spark1",
    },
    ParticleTexture {
        filename: "spark2.png",
        ui_label: "Spark2",
    },
    ParticleTexture {
        filename: "spark3.png",
        ui_label: "Spark3",
    },
    ParticleTexture {
        filename: "glow1.png",
        ui_label: "Glow1",
    },
    ParticleTexture {
        filename: "splat1.png",
        ui_label: "Splat1",
    },
];
