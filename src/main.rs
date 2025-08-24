/// OMAGARI - A Bevy-Hanabi 3D particle effects editor for HEXROLL
use bevy::{
    core_pipeline::bloom::Bloom,
    prelude::*,
    render::{camera::Viewport, view::RenderLayers},
    window::PrimaryWindow,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use bevy_egui::{
    egui::{self, scroll_area::ScrollBarVisibility, Layout},
    EguiContext, EguiContexts, EguiGlobalSettings, EguiPlugin, EguiPrimaryContextPass,
    PrimaryEguiContext,
};
use bevy_hanabi::prelude::*;
use ron::ser::PrettyConfig;
use std::fs::File;
use std::io::Write;

mod controller;
mod effect;
mod expr;
mod helpers;
mod modifiers;

use crate::controller::*;
use crate::effect::*;
use crate::expr::*;
use crate::helpers::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugins(HanabiPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(EguiPrimaryContextPass, app_ui)
        .insert_resource(OmagariProject::default())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
) {
    egui_global_settings.auto_create_primary_context = false;
    commands.spawn((
        Transform::from_xyz(0.0, 1.5, 5.0),
        Bloom::NATURAL,
        Camera {
            hdr: true,
            ..default()
        },
        PanOrbitCamera::default(),
    ));

    commands.spawn((
        PrimaryEguiContext,
        Camera2d,
        RenderLayers::none(),
        Camera {
            hdr: true,
            order: 1,
            ..default()
        },
    ));

    commands.insert_resource(EffectResource {
        effect_handles: Vec::new(),
        textures: PARTICLE_TEXTURES
            .iter()
            .map(|v| asset_server.load(v.filename))
            .collect(),
        context: AppContext::default(),
    });
}

#[derive(Default)]
struct AppContext {
    expr_clipboard: Option<ExprWriterEditor>,
    visible_effects: Vec<String>,
    filename: Option<String>,
}

fn app_ui(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut camera: Single<&mut Camera, Without<EguiContext>>,
    window: Single<&mut Window, With<PrimaryWindow>>,
    project: ResMut<OmagariProject>,
    mut res: ResMut<EffectResource>,
    effects: ResMut<Assets<EffectAsset>>,
    curr: Query<Entity, With<ParticleEffect>>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    res.context.visible_effects = project
        .effects
        .iter()
        .map(|e| e.name().to_string())
        .collect();

    let project = std::rc::Rc::new(std::cell::RefCell::new(project.into_inner()));

    let mut filename = res
        .context
        .filename
        .as_ref()
        .unwrap_or(&"".to_string())
        .clone();

    egui::TopBottomPanel::top("Toolbar")
        .resizable(false)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(2.0);
                ui.horizontal(|ui| {
                    if ui.button("⏵ RENDER").clicked() {
                        spawn_particle_effects(
                            &mut commands,
                            &mut res,
                            project.clone(),
                            effects,
                            curr,
                        );
                    }

                    if ui.button("⏹ STOP").clicked() {
                        despawn_all_particle_effects(
                            &curr,
                            &mut commands,
                        );
                    }

                    if ui.button("🖭 EXPORT").clicked() {
                        export_effects_to_files(&filename, project.clone());
                    }

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);

                    ui.label("filename:");

                    let filename_okay = validate_project_filename(&filename);

                    let mut filename_textedit =
                        egui::TextEdit::singleline(&mut filename).hint_text("filename.omagari.ron");
                    if !filename_okay {
                        filename_textedit = filename_textedit
                            .background_color(egui::Color32::from_hex("#220000").unwrap())
                    }

                    filename_textedit.show(ui);
                    res.context.filename = Some(filename.clone());

                    if ui.button("🌌 NEW").clicked() {
                        res.context.filename = None;
                        commands.insert_resource(OmagariProject::default());
                    }

                    ui.menu_button("⮉ LOAD", |ui| {
                        for f in projects_list() {
                            if ui.button(f.clone()).clicked() {
                                if let Ok(project) = load_project(&f) {
                                    commands.insert_resource(project);
                                    res.context.filename = Some(f.clone());
                                    ui.close_menu();
                                }
                            }
                        }
                    });
                    if ui
                        .add_enabled(filename_okay, egui::Button::new("⮋ SAVE"))
                        .clicked()
                    {
                        let ron_string = ron::ser::to_string_pretty(
                            *project.clone().borrow(),
                            PrettyConfig::new().new_line("\n".to_string()),
                        )
                        .unwrap();

                        let file_path = Folder::SavedEffects.full_file_path(filename);
                        if let Ok(mut file) = File::create(file_path) {
                            file.write_all(ron_string.as_bytes()).unwrap();
                        }
                    }

                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.colored_label(egui::Color32::from_hex("#88AAFF").unwrap(), "OMAGARI🔥");
                    });
                });
            });
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();

    let left = egui::SidePanel::left("EffectsPanel")
        .resizable(true)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            let effects = &mut project.borrow_mut().effects;
                            let n_effects = effects.len();
                            for (index, effect) in effects.iter_mut().enumerate() {
                                let swap = ui
                                    .horizontal(|ui| {
                                        if let Some(list_command) =
                                            ui_for_list_item(ui, index, n_effects)
                                        {
                                            return Some(list_command);
                                        } else {
                                            effect.draw_ui(&mut res.context, ui, index as u64);
                                        }
                                        None
                                    })
                                    .inner;
                                if let Some(swap) = swap {
                                    match swap {
                                        ListCommand::Remove(i) => {
                                            effects.remove(i);
                                        }
                                        ListCommand::Swap((a, b)) => {
                                            effects.swap(a, b);
                                        }
                                    }
                                    break;
                                }
                            }
                            if ui.button("+").clicked() {
                                effects.push(EffectEditor::default());
                            }
                        });
                        ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
                    })
                });

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();

    let pos = UVec2::new(left as u32, 0);
    let size =
        UVec2::new(window.physical_width(), window.physical_height()) - UVec2::new(left as u32, 0);
    camera.viewport = Some(Viewport {
        physical_position: pos,
        physical_size: size,
        ..default()
    });

    Ok(())
}
