use bevy::prelude::*;
use bevy_egui::*;
use bevy_hanabi::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::editor_prelude::AppContext;
use crate::expr::*;
use crate::helpers::*;

pub trait ModifierProducer<T>
where
    T: bevy_hanabi::Modifier,
{
    fn produce(&self, writer: &bevy_hanabi::ExprWriter) -> T;
}

pub trait RenderModifierProducer<T>
where
    T: bevy_hanabi::Modifier,
{
    type Output;
    fn produce(&self) -> T;
}

#[derive(Serialize, Deserialize)]
pub struct SetAttributeModifierEditor {
    attr: Attribute,
    attr_expr: ExprWriterEditor,
}

impl SetAttributeModifierEditor {
    pub fn label() -> &'static str {
        "🗠 SetAttributeModifier"
    }
}

impl ModifierProducer<SetAttributeModifier> for SetAttributeModifierEditor {
    fn produce(&self, writer: &ExprWriter) -> SetAttributeModifier {
        SetAttributeModifier {
            attribute: self.attr,
            value: self.attr_expr.produce(writer).expr(),
        }
    }
}

impl UiProvider for SetAttributeModifierEditor {
    fn draw_ui(&mut self, app: &mut AppContext, ui: &mut egui::Ui, index: u64) {
        let selected_text = attr_to_label(self.attr);
        let id = ui.make_persistent_id(format!("{} {}", selected_text, index));
        ui.vertical(|ui| {
            egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false)
                .show_header(ui, |ui| {
                    egui::ComboBox::new(id, "modifier")
                        .selected_text(selected_text)
                        .show_ui(ui, |ui| {
                            ALL_ATTRS.iter().for_each(|(attr, label)| {
                                ui.selectable_value(&mut self.attr, *attr, *label);
                            });
                        });
                })
                .body(|ui| {
                    self.attr_expr.draw_ui(app, ui, 1);
                });
        });
    }
}

impl Default for SetAttributeModifierEditor {
    fn default() -> Self {
        Self {
            attr: Attribute::ID,
            attr_expr: ExprWriterEditor::Float(0.0),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct InheritAttributeModifierEditor {
    attr: Attribute,
}

impl InheritAttributeModifierEditor {
    pub fn label() -> &'static str {
        "⛓ InheritAttributeModifier"
    }
}

impl ModifierProducer<InheritAttributeModifier> for InheritAttributeModifierEditor {
    fn produce(&self, _writer: &ExprWriter) -> InheritAttributeModifier {
        InheritAttributeModifier {
            attribute: self.attr,
        }
    }
}

impl UiProvider for InheritAttributeModifierEditor {
    fn draw_ui(&mut self, _app: &mut AppContext, ui: &mut egui::Ui, index: u64) {
        let selected_text = attr_to_label(self.attr);
        let id = ui.make_persistent_id(format!("{} {}", selected_text, index));
        ui.vertical(|ui| {
            egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false)
                .show_header(ui, |ui| {
                    egui::ComboBox::new(id, "is inherited")
                        .selected_text(selected_text)
                        .show_ui(ui, |ui| {
                            ALL_ATTRS.iter().for_each(|(attr, label)| {
                                ui.selectable_value(&mut self.attr, *attr, *label);
                            });
                        });
                })
                .body(|_| {});
        });
    }
}

impl Default for InheritAttributeModifierEditor {
    fn default() -> Self {
        Self {
            attr: Attribute::ID,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SetVelocityCircleModifierEditor {
    center_expr: ExprWriterEditor,
    axis_expr: ExprWriterEditor,
    speed_expr: ExprWriterEditor,
}

impl SetVelocityCircleModifierEditor {
    pub fn label() -> &'static str {
        "⏱ SetVelocityCircleModifier"
    }
}

impl ModifierProducer<SetVelocityCircleModifier> for SetVelocityCircleModifierEditor {
    fn produce(&self, writer: &ExprWriter) -> SetVelocityCircleModifier {
        SetVelocityCircleModifier {
            center: self.center_expr.produce(writer).expr(),
            axis: self.axis_expr.produce(writer).expr(),
            speed: self.speed_expr.produce(writer).expr(),
        }
    }
}

impl UiProvider for SetVelocityCircleModifierEditor {
    fn draw_ui(&mut self, app: &mut AppContext, ui: &mut egui::Ui, index: u64) {
        unique_collapsing(index, Self::label(), ui).show(ui, |ui| {
            unique_collapsing(1, "Origin", ui).show(ui, |ui| {
                self.center_expr.draw_ui(app, ui, 1);
            });
            unique_collapsing(2, "Axis", ui).show(ui, |ui| {
                self.axis_expr.draw_ui(app, ui, 2);
            });
            unique_collapsing(3, "Speed", ui).show(ui, |ui| {
                self.speed_expr.draw_ui(app, ui, 3);
            });
        });
    }
}

impl Default for SetVelocityCircleModifierEditor {
    fn default() -> Self {
        Self {
            center_expr: ExprWriterEditor::Vec3(Vec3::ZERO),
            axis_expr: ExprWriterEditor::Vec3(Vec3::Y),
            speed_expr: ExprWriterEditor::Float(0.5),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SetVelocitySphereModifierEditor {
    center_expr: ExprWriterEditor,
    speed_expr: ExprWriterEditor,
}

impl SetVelocitySphereModifierEditor {
    pub fn label() -> &'static str {
        "⏱ SetVelocitySphereModifier"
    }
}

impl ModifierProducer<SetVelocitySphereModifier> for SetVelocitySphereModifierEditor {
    fn produce(&self, writer: &ExprWriter) -> SetVelocitySphereModifier {
        SetVelocitySphereModifier {
            center: self.center_expr.produce(writer).expr(),
            speed: self.speed_expr.produce(writer).expr(),
        }
    }
}

impl UiProvider for SetVelocitySphereModifierEditor {
    fn draw_ui(&mut self, app: &mut AppContext, ui: &mut egui::Ui, index: u64) {
        unique_collapsing(index, Self::label(), ui).show(ui, |ui| {
            unique_collapsing(1, "Origin", ui).show(ui, |ui| {
                self.center_expr.draw_ui(app, ui, 1);
            });
            unique_collapsing(3, "Speed", ui).show(ui, |ui| {
                self.speed_expr.draw_ui(app, ui, 3);
            });
        });
    }
}

impl Default for SetVelocitySphereModifierEditor {
    fn default() -> Self {
        Self {
            center_expr: ExprWriterEditor::Vec3(Vec3::ZERO),
            speed_expr: ExprWriterEditor::Float(0.5),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SetVelocityTangentModifierEditor {
    origin_expr: ExprWriterEditor,
    axis_expr: ExprWriterEditor,
    speed_expr: ExprWriterEditor,
}

impl SetVelocityTangentModifierEditor {
    pub fn label() -> &'static str {
        "⏱ SetVelocityTangentModifier"
    }
}

impl ModifierProducer<SetVelocityTangentModifier> for SetVelocityTangentModifierEditor {
    fn produce(&self, writer: &ExprWriter) -> SetVelocityTangentModifier {
        SetVelocityTangentModifier {
            origin: self.origin_expr.produce(writer).expr(),
            axis: self.axis_expr.produce(writer).expr(),
            speed: self.speed_expr.produce(writer).expr(),
        }
    }
}

impl UiProvider for SetVelocityTangentModifierEditor {
    fn draw_ui(&mut self, app: &mut AppContext, ui: &mut egui::Ui, index: u64) {
        unique_collapsing(index, Self::label(), ui).show(ui, |ui| {
            unique_collapsing(index, "Origin", ui).show(ui, |ui| {
                self.origin_expr.draw_ui(app, ui, 1);
            });
            unique_collapsing(index, "Axis", ui).show(ui, |ui| {
                self.axis_expr.draw_ui(app, ui, 2);
            });
            unique_collapsing(index, "Speed", ui).show(ui, |ui| {
                self.speed_expr.draw_ui(app, ui, 3);
            });
        });
    }
}

impl Default for SetVelocityTangentModifierEditor {
    fn default() -> Self {
        Self {
            origin_expr: ExprWriterEditor::Vec3(Vec3::ZERO),
            axis_expr: ExprWriterEditor::Vec3(Vec3::Y),
            speed_expr: ExprWriterEditor::Operator(Box::new(ExprOperatorEditor::Uniform(
                ExprWriterEditor::Float(0.2),
                ExprWriterEditor::Float(1.0),
            ))),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SetPositionCircleModifierEditor {
    center_expr: ExprWriterEditor,
    axis_expr: ExprWriterEditor,
    radius_expr: ExprWriterEditor,
    dimension: ShapeDimension,
}

impl SetPositionCircleModifierEditor {
    pub fn label() -> &'static str {
        "🕂 SetPositionCircleModifier"
    }
}

impl ModifierProducer<SetPositionCircleModifier> for SetPositionCircleModifierEditor {
    fn produce(&self, writer: &ExprWriter) -> SetPositionCircleModifier {
        SetPositionCircleModifier {
            center: self.center_expr.produce(writer).expr(),
            axis: self.axis_expr.produce(writer).expr(),
            radius: self.radius_expr.produce(writer).expr(),
            dimension: self.dimension,
        }
    }
}

impl UiProvider for SetPositionCircleModifierEditor {
    fn draw_ui(&mut self, app: &mut AppContext, ui: &mut egui::Ui, index: u64) {
        unique_collapsing(index, Self::label(), ui).show(ui, |ui| {
            unique_collapsing(index, "Center", ui).show(ui, |ui| {
                self.center_expr.draw_ui(app, ui, 1);
            });
            unique_collapsing(index, "Axis", ui).show(ui, |ui| {
                self.axis_expr.draw_ui(app, ui, 2);
            });
            unique_collapsing(index, "Radius", ui).show(ui, |ui| {
                self.radius_expr.draw_ui(app, ui, 3);
            });
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.dimension, ShapeDimension::Surface, "Surface");
                ui.radio_value(&mut self.dimension, ShapeDimension::Volume, "Volume");
            });
        });
    }
}

impl Default for SetPositionCircleModifierEditor {
    fn default() -> Self {
        Self {
            center_expr: ExprWriterEditor::Vec3(Vec3::ZERO),
            axis_expr: ExprWriterEditor::Vec3(Vec3::Y),
            radius_expr: ExprWriterEditor::Float(0.2),
            dimension: ShapeDimension::Surface,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SetPositionSphereModifierEditor {
    center_expr: ExprWriterEditor,
    radius_expr: ExprWriterEditor,
    dimension: ShapeDimension,
}

impl SetPositionSphereModifierEditor {
    pub fn label() -> &'static str {
        "🕂 SetPositionSphereModifier"
    }
}

impl ModifierProducer<SetPositionSphereModifier> for SetPositionSphereModifierEditor {
    fn produce(&self, writer: &ExprWriter) -> SetPositionSphereModifier {
        SetPositionSphereModifier {
            center: self.center_expr.produce(writer).expr(),
            radius: self.radius_expr.produce(writer).expr(),
            dimension: self.dimension,
        }
    }
}

impl UiProvider for SetPositionSphereModifierEditor {
    fn draw_ui(&mut self, app: &mut AppContext, ui: &mut egui::Ui, index: u64) {
        unique_collapsing(index, Self::label(), ui).show(ui, |ui| {
            unique_collapsing(index, "Center", ui).show(ui, |ui| {
                self.center_expr.draw_ui(app, ui, 1);
            });
            unique_collapsing(index, "Radius", ui).show(ui, |ui| {
                self.radius_expr.draw_ui(app, ui, 3);
            });
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.dimension, ShapeDimension::Surface, "Surface");
                ui.radio_value(&mut self.dimension, ShapeDimension::Volume, "Volume");
            });
        });
    }
}

impl Default for SetPositionSphereModifierEditor {
    fn default() -> Self {
        Self {
            center_expr: ExprWriterEditor::Vec3(Vec3::ZERO),
            radius_expr: ExprWriterEditor::Float(0.2),
            dimension: ShapeDimension::Surface,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AccelModifierEditor {
    accel_expr: ExprWriterEditor,
}

impl AccelModifierEditor {
    pub fn label() -> &'static str {
        "🕛 AccelModifier"
    }
}

impl ModifierProducer<AccelModifier> for AccelModifierEditor {
    fn produce(&self, writer: &ExprWriter) -> AccelModifier {
        AccelModifier::new(self.accel_expr.produce(writer).expr())
    }
}

impl UiProvider for AccelModifierEditor {
    fn draw_ui(&mut self, app: &mut AppContext, ui: &mut egui::Ui, index: u64) {
        unique_collapsing(index, Self::label(), ui).show(ui, |ui| {
            unique_collapsing(index, "Origin", ui).show(ui, |ui| {
                self.accel_expr.draw_ui(app, ui, 1);
            });
        });
    }
}

impl Default for AccelModifierEditor {
    fn default() -> Self {
        Self {
            accel_expr: ExprWriterEditor::Operator(Box::new(ExprOperatorEditor::Subtract(
                ExprWriterEditor::Operator(Box::new(ExprOperatorEditor::Multiply(
                    ExprWriterEditor::RandVec3,
                    ExprWriterEditor::Vec3(Vec3::splat(2.0)),
                ))),
                ExprWriterEditor::Vec3(Vec3::ONE),
            ))),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LinearDragModifierEditor {
    drag_expr: ExprWriterEditor,
}

impl LinearDragModifierEditor {
    pub fn label() -> &'static str {
        "☄ LinearDragModifier"
    }
}

impl ModifierProducer<LinearDragModifier> for LinearDragModifierEditor {
    fn produce(&self, writer: &ExprWriter) -> LinearDragModifier {
        LinearDragModifier {
            drag: self.drag_expr.produce(writer).expr(),
        }
    }
}

impl UiProvider for LinearDragModifierEditor {
    fn draw_ui(&mut self, app: &mut AppContext, ui: &mut egui::Ui, index: u64) {
        unique_collapsing(index, Self::label(), ui).show(ui, |ui| {
            unique_collapsing(index, "Drag", ui).show(ui, |ui| {
                self.drag_expr.draw_ui(app, ui, 1);
            });
        });
    }
}

impl Default for LinearDragModifierEditor {
    fn default() -> Self {
        Self {
            drag_expr: ExprWriterEditor::Placeholder,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EmitSpawnEventModifierEditor {
    condition: EventEmitCondition,
    count_expr: ExprWriterEditor,
    child_index: u32,
}

impl EmitSpawnEventModifierEditor {
    pub fn label() -> &'static str {
        "⚡ EmitSpawnEvent"
    }
}

impl ModifierProducer<EmitSpawnEventModifier> for EmitSpawnEventModifierEditor {
    fn produce(&self, writer: &ExprWriter) -> EmitSpawnEventModifier {
        EmitSpawnEventModifier {
            condition: self.condition,
            count: self.count_expr.produce(writer).expr(),
            child_index: self.child_index,
        }
    }
}

impl UiProvider for EmitSpawnEventModifierEditor {
    fn draw_ui(&mut self, app: &mut AppContext, ui: &mut egui::Ui, index: u64) {
        unique_collapsing(index, Self::label(), ui).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Count:");
                self.count_expr.draw_ui(app, ui, 1);
            });
            ui.horizontal(|ui| {
                ui.label("Child index:");
                self.child_index = ui_for_u32_ex(ui, self.child_index, 0, 16, 1);
            });
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.condition, EventEmitCondition::Always, "Always");
                ui.radio_value(&mut self.condition, EventEmitCondition::OnDie, "OnDie");
            });
        });
    }
}

impl Default for EmitSpawnEventModifierEditor {
    fn default() -> Self {
        Self {
            condition: EventEmitCondition::OnDie,
            count_expr: ExprWriterEditor::U32(0),
            child_index: 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ConformToSphereModifierEditor {
    origin_expr: ExprWriterEditor,
    radius_expr: ExprWriterEditor,
    influence_dist_expr: ExprWriterEditor,
    attraction_accel_expr: ExprWriterEditor,
    max_attraction_speed_expr: ExprWriterEditor,
}

impl ConformToSphereModifierEditor {
    pub fn label() -> &'static str {
        "📌 ConformToSphereModifier"
    }
}

impl ModifierProducer<ConformToSphereModifier> for ConformToSphereModifierEditor {
    fn produce(&self, writer: &ExprWriter) -> ConformToSphereModifier {
        ConformToSphereModifier {
            origin: self.origin_expr.produce(writer).expr(),
            radius: self.radius_expr.produce(writer).expr(),
            influence_dist: self.influence_dist_expr.produce(writer).expr(),
            attraction_accel: self.attraction_accel_expr.produce(writer).expr(),
            max_attraction_speed: self.max_attraction_speed_expr.produce(writer).expr(),
            shell_half_thickness: None,
            sticky_factor: None,
        }
    }
}

impl UiProvider for ConformToSphereModifierEditor {
    fn draw_ui(&mut self, app: &mut AppContext, ui: &mut egui::Ui, index: u64) {
        unique_collapsing(index, Self::label(), ui).show(ui, |ui| {
            unique_collapsing(1, "Origin", ui).show(ui, |ui| {
                self.origin_expr.draw_ui(app, ui, 1);
            });
            unique_collapsing(2, "Radius", ui).show(ui, |ui| {
                self.radius_expr.draw_ui(app, ui, 1);
            });
            unique_collapsing(3, "Influence Distance", ui).show(ui, |ui| {
                self.influence_dist_expr.draw_ui(app, ui, 1);
            });
            unique_collapsing(4, "Attraction Acceleration", ui).show(ui, |ui| {
                self.attraction_accel_expr.draw_ui(app, ui, 1);
            });
            unique_collapsing(5, "Max Attraction Speed", ui).show(ui, |ui| {
                self.max_attraction_speed_expr.draw_ui(app, ui, 1);
            });
        });
    }
}

impl Default for ConformToSphereModifierEditor {
    fn default() -> Self {
        Self {
            origin_expr: ExprWriterEditor::Vec3(Vec3::ZERO),
            radius_expr: ExprWriterEditor::Float(1.0),
            influence_dist_expr: ExprWriterEditor::Float(10.0),
            attraction_accel_expr: ExprWriterEditor::Float(2.0),
            max_attraction_speed_expr: ExprWriterEditor::Float(2.0),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SizeOverLifetimeModifierEditor {
    gradient: GradientVec3Editor,
}

impl SizeOverLifetimeModifierEditor {
    pub fn label() -> &'static str {
        "📏 SizeOverLifetime"
    }
}

impl RenderModifierProducer<SizeOverLifetimeModifier> for SizeOverLifetimeModifierEditor {
    type Output = SizeOverLifetimeModifier;

    fn produce(&self) -> SizeOverLifetimeModifier {
        SizeOverLifetimeModifier {
            gradient: self.gradient.produce(),
            screen_space_size: false,
        }
    }
}

impl UiProvider for SizeOverLifetimeModifierEditor {
    fn draw_ui(&mut self, app: &mut AppContext, ui: &mut egui::Ui, index: u64) {
        unique_collapsing(index, Self::label(), ui).show(ui, |ui| {
            unique_collapsing(1, "Gradient", ui).show(ui, |ui| self.gradient.draw_ui(app, ui, 1));
        });
    }
}

impl Default for SizeOverLifetimeModifierEditor {
    fn default() -> Self {
        Self {
            gradient: GradientVec3Editor {
                g: vec![(0.3, Vec3::splat(0.1)), (1.0, Vec3::splat(1.0))],
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ColorOverLifetimeModifierEditor {
    gradient: GradientVec4Editor,
    blend: Option<ColorBlendMode>,
    mask: Option<ColorBlendMask>,
}

impl ColorOverLifetimeModifierEditor {
    pub fn label() -> &'static str {
        "🎨 ColorOverLifetime"
    }
}

impl RenderModifierProducer<ColorOverLifetimeModifier> for ColorOverLifetimeModifierEditor {
    type Output = ColorOverLifetimeModifier;

    fn produce(&self) -> Self::Output {
        ColorOverLifetimeModifier {
            gradient: self.gradient.produce(),
            blend: self.blend.unwrap_or(ColorBlendMode::default()),
            mask: self.mask.unwrap_or(ColorBlendMask::default()),
        }
    }
}

impl UiProvider for ColorOverLifetimeModifierEditor {
    fn draw_ui(&mut self, app: &mut AppContext, ui: &mut egui::Ui, index: u64) {
        unique_collapsing(index, Self::label(), ui).show(ui, |ui| {
            unique_collapsing(1, "Gradient", ui).show(ui, |ui| self.gradient.draw_ui(app, ui, 0));
            unique_collapsing(2, "Blend", ui).show(ui, |ui| {
                let mut blend = self.blend.unwrap_or(ColorBlendMode::default());
                ui.horizontal(|ui| {
                    ui.radio_value(&mut blend, ColorBlendMode::Add, "Add");
                    ui.radio_value(&mut blend, ColorBlendMode::Modulate, "Modulate");
                    ui.radio_value(&mut blend, ColorBlendMode::Overwrite, "Overwrite");
                });
                self.blend = Some(blend);
            });
            unique_collapsing(3, "Mask", ui).show(ui, |ui| {
                let mut mask = self.mask.unwrap_or(ColorBlendMask::default());
                ui.horizontal(|ui| {
                    ui.radio_value(&mut mask, ColorBlendMask::RGB, "RGB");
                    ui.radio_value(&mut mask, ColorBlendMask::RGBA, "RGBA");
                });
                self.mask = Some(mask);
            });
        });
    }
}

impl Default for ColorOverLifetimeModifierEditor {
    fn default() -> Self {
        Self {
            gradient: GradientVec4Editor {
                g: vec![
                    (0.0, Vec4::new(0.0, 4.0, 4.0, 0.0)),
                    (0.1, Vec4::new(0.0, 4.0, 4.0, 1.0)),
                    (0.3, Vec4::new(4.0, 4.0, 0.0, 1.0)),
                    (0.6, Vec4::new(4.0, 0.0, 0.0, 0.0)),
                    (1.0, Vec4::new(4.0, 0.0, 0.0, 0.0)),
                ],
            },
            blend: Some(ColorBlendMode::default()),
            mask: Some(ColorBlendMask::default()),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct GradientVec3Editor {
    g: Vec<(f32, Vec3)>,
}

impl GradientVec3Editor {
    pub fn produce(&self) -> Gradient<Vec3> {
        let mut gradient = Gradient::<Vec3>::new();
        for (t, v) in self.g.iter() {
            gradient.add_key(*t, *v);
        }
        gradient
    }
}

impl UiProvider for GradientVec3Editor {
    fn draw_ui(&mut self, _app: &mut AppContext, ui: &mut egui::Ui, _index: u64) {
        let n_keys = self.g.len();
        for (index, (t, v)) in self.g.iter_mut().enumerate() {
            if let Some(list_command) = ui
                .horizontal(|ui| {
                    if let Some(list_command) = ui_for_list_item(ui, index, n_keys) {
                        return Some(list_command);
                    } else {
                        ui.label("t:");
                        *t = ui_for_f32(ui, *t);
                        ui.label("vec3:");
                        *v = ui_for_vec3(ui, *v);
                        None
                    }
                })
                .inner
            {
                list_command.apply(&mut self.g);
                break;
            }
        }
        if ui.button("+").clicked() {
            self.g.push((0.0, Vec3::ZERO));
        }
    }
}

#[derive(Serialize, Deserialize)]
struct GradientVec4Editor {
    g: Vec<(f32, Vec4)>,
}

impl GradientVec4Editor {
    pub fn produce(&self) -> Gradient<Vec4> {
        let mut gradient = Gradient::<Vec4>::new();
        for (t, v) in self.g.iter() {
            gradient.add_key(*t, *v);
        }
        gradient
    }
}

impl UiProvider for GradientVec4Editor {
    fn draw_ui(&mut self, _app: &mut AppContext, ui: &mut egui::Ui, _index: u64) {
        let n_keys = self.g.len();
        for (index, (t, v)) in self.g.iter_mut().enumerate() {
            if let Some(list_command) = ui
                .horizontal(|ui| {
                    if let Some(list_command) = ui_for_list_item(ui, index, n_keys) {
                        return Some(list_command);
                    } else {
                        ui.label("t:");
                        *t = ui_for_f32(ui, *t);
                        ui.label("vec4:");
                        *v = ui_for_vec4(ui, *v);
                    }
                    None
                })
                .inner
            {
                list_command.apply(&mut self.g);
                break;
            }
        }
        if ui.button("+").clicked() {
            self.g.push((0.0, Vec4::ZERO));
        }
    }
}
