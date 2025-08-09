use bevy::prelude::*;

use bevy_egui::*;
use bevy_hanabi::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::editor_prelude::AppContext;
use crate::helpers::*;

pub const ALL_ATTRS: [(Attribute, &str); 39] = [
    (Attribute::ID, "ID"),
    (Attribute::PARTICLE_COUNTER, "Particle Counter"),
    (Attribute::POSITION, "Position"),
    (Attribute::VELOCITY, "Velocity"),
    (Attribute::AGE, "Age"),
    (Attribute::LIFETIME, "Lifetime"),
    (Attribute::COLOR, "Color"),
    (Attribute::HDR_COLOR, "HDR Color"),
    (Attribute::ALPHA, "Alpha"),
    (Attribute::SIZE, "Size"),
    (Attribute::SIZE2, "Size2"),
    (Attribute::SIZE3, "Size3"),
    (Attribute::PREV, "Prev"),
    (Attribute::NEXT, "Next"),
    (Attribute::AXIS_X, "Axis X"),
    (Attribute::AXIS_Y, "Axis Y"),
    (Attribute::AXIS_Z, "Axis Z"),
    (Attribute::SPRITE_INDEX, "Sprite Index"),
    (Attribute::F32_0, "F32_0"),
    (Attribute::F32_1, "F32_1"),
    (Attribute::F32_2, "F32_2"),
    (Attribute::F32_3, "F32_3"),
    (Attribute::F32X2_0, "F32X2_0"),
    (Attribute::F32X2_1, "F32X2_1"),
    (Attribute::F32X2_2, "F32X2_2"),
    (Attribute::F32X2_3, "F32X2_3"),
    (Attribute::F32X3_0, "F32X3_0"),
    (Attribute::F32X3_1, "F32X3_1"),
    (Attribute::F32X3_2, "F32X3_2"),
    (Attribute::F32X3_3, "F32X3_3"),
    (Attribute::F32X4_0, "F32X4_0"),
    (Attribute::F32X4_1, "F32X4_1"),
    (Attribute::F32X4_2, "F32X4_2"),
    (Attribute::F32X4_3, "F32X4_3"),
    (Attribute::U32_0, "U32_0"),
    (Attribute::U32_1, "U32_1"),
    (Attribute::U32_2, "U32_2"),
    (Attribute::U32_3, "U32_3"),
    (Attribute::RIBBON_ID, "Ribbon ID"),
];

pub fn attr_to_label(attr: Attribute) -> &'static str {
    if let Some(result) = ALL_ATTRS
        .iter()
        .filter(|(attr_opt, _)| attr == *attr_opt)
        .next()
    {
        result.1
    } else {
        "None"
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ExprOperatorEditor {
    Uniform(ExprWriterEditor, ExprWriterEditor),
    Multiply(ExprWriterEditor, ExprWriterEditor),
    Subtract(ExprWriterEditor, ExprWriterEditor),
    Add(ExprWriterEditor, ExprWriterEditor),
    Sin(ExprWriterEditor),
    Cos(ExprWriterEditor),
    Distance(ExprWriterEditor, ExprWriterEditor),
    Vec3(ExprWriterEditor, ExprWriterEditor, ExprWriterEditor),
    Vec4(
        ExprWriterEditor,
        ExprWriterEditor,
        ExprWriterEditor,
        ExprWriterEditor,
    ),
    Pack4x8UNorm(ExprWriterEditor),
    Attr(Attribute),
    ParentAttr(Attribute),
    Normalized(ExprWriterEditor),
}

impl ExprOperatorEditor {
    pub fn produce(&self, writer: &ExprWriter) -> WriterExpr {
        match self {
            ExprOperatorEditor::Uniform(lit1, lit2) => {
                lit1.produce(writer).uniform(lit2.produce(writer))
            }
            ExprOperatorEditor::Multiply(lit1, lit2) => {
                lit1.produce(writer).mul(lit2.produce(writer))
            }
            ExprOperatorEditor::Subtract(lit1, lit2) => {
                lit1.produce(writer).sub(lit2.produce(writer))
            }
            ExprOperatorEditor::Add(lit1, lit2) => lit1.produce(writer).add(lit2.produce(writer)),
            ExprOperatorEditor::Distance(lit1, lit2) => {
                lit1.produce(writer).distance(lit2.produce(writer))
            }
            ExprOperatorEditor::Sin(lit) => lit.produce(writer).sin(),
            ExprOperatorEditor::Cos(lit) => lit.produce(writer).cos(),
            ExprOperatorEditor::Vec3(lit1, lit2, lit3) => lit1
                .produce(writer)
                .vec3(lit2.produce(writer), lit3.produce(writer)),
            ExprOperatorEditor::Vec4(lit1, lit2, lit3, lit4) => lit1
                .produce(writer)
                .vec3(lit2.produce(writer), lit3.produce(writer))
                .vec4_xyz_w(lit4.produce(writer)),
            ExprOperatorEditor::Pack4x8UNorm(lit) => lit.produce(writer).pack4x8unorm(),
            ExprOperatorEditor::Attr(attr) => writer.attr(*attr),
            ExprOperatorEditor::ParentAttr(attr) => writer.parent_attr(*attr),
            ExprOperatorEditor::Normalized(lit) => lit.produce(writer).normalized(),
        }
    }

    pub fn draw_ui(&mut self, app: &mut AppContext, ui: &mut egui::Ui, index: u64) {
        let unique_id = index;
        match self {
            ExprOperatorEditor::Uniform(lit1, lit2) => {
                unique_collapsing(unique_id, "🖩 Uniform", ui).show(ui, |ui| {
                    lit1.draw_ui(app, ui, 1);
                    lit2.draw_ui(app, ui, 2);
                });
            }
            ExprOperatorEditor::Multiply(lit1, lit2) => {
                unique_collapsing(unique_id, "🖩 Multiply", ui).show(ui, |ui| {
                    lit1.draw_ui(app, ui, 1);
                    lit2.draw_ui(app, ui, 2);
                });
            }
            ExprOperatorEditor::Subtract(lit1, lit2) => {
                unique_collapsing(unique_id, "🖩 Subtract", ui).show(ui, |ui| {
                    lit1.draw_ui(app, ui, 1);
                    lit2.draw_ui(app, ui, 2);
                });
            }
            ExprOperatorEditor::Add(lit1, lit2) => {
                unique_collapsing(unique_id, "🖩 Add", ui).show(ui, |ui| {
                    lit1.draw_ui(app, ui, 1);
                    lit2.draw_ui(app, ui, 2);
                });
            }
            ExprOperatorEditor::Sin(lit) => {
                unique_collapsing(unique_id, "🖩 Sin", ui).show(ui, |ui| {
                    lit.draw_ui(app, ui, 1);
                });
            }
            ExprOperatorEditor::Cos(lit) => {
                unique_collapsing(unique_id, "🖩 Cos", ui).show(ui, |ui| {
                    lit.draw_ui(app, ui, 1);
                });
            }
            ExprOperatorEditor::Distance(lit1, lit2) => {
                unique_collapsing(unique_id, "🖩 Distance", ui).show(ui, |ui| {
                    lit1.draw_ui(app, ui, 1);
                    lit2.draw_ui(app, ui, 2);
                });
            }
            ExprOperatorEditor::Vec3(lit1, lit2, lit3) => {
                unique_collapsing(unique_id, "🖩 Vec3", ui).show(ui, |ui| {
                    lit1.draw_ui(app, ui, 1);
                    lit2.draw_ui(app, ui, 2);
                    lit3.draw_ui(app, ui, 3);
                });
            }
            ExprOperatorEditor::Vec4(lit1, lit2, lit3, lit4) => {
                unique_collapsing(unique_id, "🖩 Vec4", ui).show(ui, |ui| {
                    lit1.draw_ui(app, ui, 1);
                    lit2.draw_ui(app, ui, 2);
                    lit3.draw_ui(app, ui, 3);
                    lit4.draw_ui(app, ui, 4);
                });
            }
            ExprOperatorEditor::Normalized(lit1) => {
                unique_collapsing(unique_id, "🖩 Normalized", ui).show(ui, |ui| {
                    lit1.draw_ui(app, ui, 1);
                });
            }
            ExprOperatorEditor::Pack4x8UNorm(lit1) => {
                unique_collapsing(unique_id, "🖩 Pack4x8UNorm", ui).show(ui, |ui| {
                    lit1.draw_ui(app, ui, 1);
                });
            }
            ExprOperatorEditor::Attr(attr) | ExprOperatorEditor::ParentAttr(attr) => {
                let mut selected_attr: Attribute = attr.clone();
                let label = match self {
                    ExprOperatorEditor::Attr(_) => "🖩 Attr",
                    ExprOperatorEditor::ParentAttr(_) => "🖩 ParentAttr",
                    _ => unreachable!(),
                };
                unique_collapsing(unique_id, label, ui).show(ui, |ui| {
                    let selected_text = attr_to_label(selected_attr);

                    let id = ui.make_persistent_id(format!("header"));
                    ui.vertical(|ui| {
                        egui::collapsing_header::CollapsingState::load_with_default_open(
                            ui.ctx(),
                            id,
                            false,
                        )
                        .show_header(ui, |ui| {
                            egui::ComboBox::new(id, "is inherited")
                                .selected_text(selected_text)
                                .show_ui(ui, |ui| {
                                    ALL_ATTRS.iter().for_each(|(attr, label)| {
                                        ui.selectable_value(&mut selected_attr, *attr, *label);
                                    });
                                });
                        })
                        .body(|_| {});
                    });
                });
                *self = if let ExprOperatorEditor::Attr(_) = self {
                    ExprOperatorEditor::Attr(selected_attr)
                } else {
                    ExprOperatorEditor::ParentAttr(selected_attr)
                };
            }
        }
    }

    pub fn menu_ui(ui: &mut egui::Ui) -> Option<Box<ExprOperatorEditor>> {
        if ui.button("🖩 Uniform").clicked() {
            return Some(Box::new(ExprOperatorEditor::Uniform(
                ExprWriterEditor::Placeholder,
                ExprWriterEditor::Placeholder,
            )));
        }
        if ui.button("🖩 Multiply").clicked() {
            return Some(Box::new(ExprOperatorEditor::Multiply(
                ExprWriterEditor::Placeholder,
                ExprWriterEditor::Placeholder,
            )));
        }
        if ui.button("🖩 Subtract").clicked() {
            return Some(Box::new(ExprOperatorEditor::Subtract(
                ExprWriterEditor::Placeholder,
                ExprWriterEditor::Placeholder,
            )));
        }
        if ui.button("🖩 Add").clicked() {
            return Some(Box::new(ExprOperatorEditor::Add(
                ExprWriterEditor::Placeholder,
                ExprWriterEditor::Placeholder,
            )));
        }
        if ui.button("🖩 Sin").clicked() {
            return Some(Box::new(ExprOperatorEditor::Sin(
                ExprWriterEditor::Placeholder,
            )));
        }
        if ui.button("🖩 Cos").clicked() {
            return Some(Box::new(ExprOperatorEditor::Cos(
                ExprWriterEditor::Placeholder,
            )));
        }
        if ui.button("🖩 Distance").clicked() {
            return Some(Box::new(ExprOperatorEditor::Distance(
                ExprWriterEditor::Placeholder,
                ExprWriterEditor::Placeholder,
            )));
        }
        if ui.button("🖩 Vec3").clicked() {
            return Some(Box::new(ExprOperatorEditor::Vec3(
                ExprWriterEditor::Placeholder,
                ExprWriterEditor::Placeholder,
                ExprWriterEditor::Placeholder,
            )));
        }
        if ui.button("🖩 Normalized").clicked() {
            return Some(Box::new(ExprOperatorEditor::Normalized(
                ExprWriterEditor::Placeholder,
            )));
        }
        if ui.button("🖩 Vec4").clicked() {
            return Some(Box::new(ExprOperatorEditor::Vec4(
                ExprWriterEditor::Placeholder,
                ExprWriterEditor::Placeholder,
                ExprWriterEditor::Placeholder,
                ExprWriterEditor::Placeholder,
            )));
        }
        if ui.button("🖩 Pack4x8UNorm").clicked() {
            return Some(Box::new(ExprOperatorEditor::Pack4x8UNorm(
                ExprWriterEditor::Placeholder,
            )));
        }
        if ui.button("🖩 Attr").clicked() {
            return Some(Box::new(ExprOperatorEditor::Attr(Attribute::ID)));
        }
        if ui.button("🖩 Parent Attr").clicked() {
            return Some(Box::new(ExprOperatorEditor::ParentAttr(Attribute::ID)));
        }
        None
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ExprWriterEditor {
    Placeholder,
    RandU32,
    RandFloat,
    RandVec3,
    Operator(Box<ExprOperatorEditor>),
    Float(f32),
    U32(u32),
    Vec3(Vec3),
    Vec4(Vec4),
    Time,
    Age,
}

impl ExprWriterEditor {
    pub fn produce(&self, writer: &ExprWriter) -> WriterExpr {
        match self {
            ExprWriterEditor::Operator(o) => o.produce(writer),
            ExprWriterEditor::RandU32 => writer.rand(ValueType::Scalar(ScalarType::Uint)),
            ExprWriterEditor::RandFloat => writer.rand(ValueType::Scalar(ScalarType::Float)),
            ExprWriterEditor::RandVec3 => writer.rand(ValueType::Vector(VectorType::VEC3F)),
            ExprWriterEditor::Float(f) => writer.lit(*f),
            ExprWriterEditor::U32(f) => writer.lit(*f),
            ExprWriterEditor::Vec3(v) => writer.lit(*v),
            ExprWriterEditor::Vec4(v) => writer.lit(*v),
            ExprWriterEditor::Time => writer.time(),
            ExprWriterEditor::Placeholder => writer.lit(0.0),
            ExprWriterEditor::Age => writer.attr(Attribute::AGE),
        }
    }
    pub fn draw_ui(&mut self, app: &mut AppContext, ui: &mut egui::Ui, index: u64) {
        let c = self.clone();
        match self {
            ExprWriterEditor::Operator(o) => {
                if let Some(v) = ui
                    .horizontal(|ui| match ui_tools_for_expr_writer("", ui) {
                        ExprControl::Delete => Some(ExprWriterEditor::Placeholder),
                        ExprControl::Noop => {
                            o.draw_ui(app, ui, index);
                            None
                        }
                        ExprControl::Copy => {
                            app.expr_clipboard = Some(c);
                            None
                        }
                    })
                    .inner
                {
                    *self = v;
                }
            }
            ExprWriterEditor::Age => {
                if let Some(v) = ui
                    .horizontal(|ui| match ui_tools_for_expr_writer("Age", ui) {
                        ExprControl::Delete => Some(ExprWriterEditor::Placeholder),
                        ExprControl::Noop => None,
                        ExprControl::Copy => {
                            app.expr_clipboard = Some(c);
                            None
                        }
                    })
                    .inner
                {
                    *self = v;
                }
            }
            ExprWriterEditor::RandU32 => {
                if let Some(v) = ui
                    .horizontal(|ui| match ui_tools_for_expr_writer("Rand U32", ui) {
                        ExprControl::Delete => Some(ExprWriterEditor::Placeholder),
                        ExprControl::Noop => None,
                        ExprControl::Copy => {
                            app.expr_clipboard = Some(c);
                            None
                        }
                    })
                    .inner
                {
                    *self = v;
                }
            }
            ExprWriterEditor::RandFloat => {
                if let Some(v) = ui
                    .horizontal(|ui| match ui_tools_for_expr_writer("Rand Float", ui) {
                        ExprControl::Delete => Some(ExprWriterEditor::Placeholder),
                        ExprControl::Noop => None,
                        ExprControl::Copy => {
                            app.expr_clipboard = Some(c);
                            None
                        }
                    })
                    .inner
                {
                    *self = v;
                }
            }
            ExprWriterEditor::RandVec3 => {
                if let Some(v) = ui
                    .horizontal(|ui| match ui_tools_for_expr_writer("Rand Vec3", ui) {
                        ExprControl::Delete => Some(ExprWriterEditor::Placeholder),
                        ExprControl::Noop => None,
                        ExprControl::Copy => {
                            app.expr_clipboard = Some(c);
                            None
                        }
                    })
                    .inner
                {
                    *self = v;
                }
            }
            ExprWriterEditor::Float(f) => {
                *self = ui
                    .horizontal(|ui| match ui_tools_for_expr_writer("Float", ui) {
                        ExprControl::Delete => ExprWriterEditor::Placeholder,
                        ExprControl::Noop => ExprWriterEditor::Float(ui_for_f32(ui, *f)),
                        ExprControl::Copy => {
                            app.expr_clipboard = Some(c);
                            ExprWriterEditor::Float(ui_for_f32(ui, *f))
                        }
                    })
                    .inner;
            }
            ExprWriterEditor::U32(f) => {
                *self = ui
                    .horizontal(|ui| match ui_tools_for_expr_writer("U32", ui) {
                        ExprControl::Delete => ExprWriterEditor::Placeholder,
                        ExprControl::Noop => {
                            ExprWriterEditor::U32(ui_for_u32_ex(ui, *f, 0, 10000, 1))
                        }
                        ExprControl::Copy => {
                            app.expr_clipboard = Some(c);
                            ExprWriterEditor::U32(ui_for_u32_ex(ui, *f, 0, 10000, 1))
                        }
                    })
                    .inner;
            }
            ExprWriterEditor::Vec3(result) => {
                *self = ui
                    .horizontal(|ui| match ui_tools_for_expr_writer("Vec3", ui) {
                        ExprControl::Delete => ExprWriterEditor::Placeholder,
                        ExprControl::Noop => ExprWriterEditor::Vec3(ui_for_vec3(ui, *result)),
                        ExprControl::Copy => {
                            app.expr_clipboard = Some(c);
                            ExprWriterEditor::Vec3(ui_for_vec3(ui, *result))
                        }
                    })
                    .inner;
            }
            ExprWriterEditor::Vec4(result) => {
                *self = ui
                    .horizontal(|ui| match ui_tools_for_expr_writer("Vec4", ui) {
                        ExprControl::Delete => ExprWriterEditor::Placeholder,
                        ExprControl::Noop => ExprWriterEditor::Vec4(ui_for_vec4(ui, *result)),
                        ExprControl::Copy => {
                            app.expr_clipboard = Some(c);
                            ExprWriterEditor::Vec4(ui_for_vec4(ui, *result))
                        }
                    })
                    .inner;
            }
            ExprWriterEditor::Time => {
                *self = ui
                    .horizontal(|ui| match ui_tools_for_expr_writer("Time", ui) {
                        ExprControl::Delete => ExprWriterEditor::Placeholder,
                        ExprControl::Noop => ExprWriterEditor::Time,
                        ExprControl::Copy => {
                            app.expr_clipboard = Some(c);
                            ExprWriterEditor::Time
                        }
                    })
                    .inner;
            }

            ExprWriterEditor::Placeholder => {
                ui.menu_button("+", |ui| {
                    ui.menu_button("Operator", |ui| {
                        if let Some(op) = ExprOperatorEditor::menu_ui(ui) {
                            *self = ExprWriterEditor::Operator(op);
                        }
                    });
                    if ui.button("Age").clicked() {
                        *self = ExprWriterEditor::Age;
                    }
                    if ui.button("RandFloat").clicked() {
                        *self = ExprWriterEditor::RandFloat;
                    }
                    if ui.button("RandU32").clicked() {
                        *self = ExprWriterEditor::RandU32;
                    }
                    if ui.button("RandVec3").clicked() {
                        *self = ExprWriterEditor::RandVec3;
                    }
                    if ui.button("Time").clicked() {
                        *self = ExprWriterEditor::Time;
                    }
                    if ui.button("Float").clicked() {
                        *self = ExprWriterEditor::Float(0.0);
                    }
                    if ui.button("U32").clicked() {
                        *self = ExprWriterEditor::U32(0);
                    }
                    if ui.button("Vec3").clicked() {
                        *self = ExprWriterEditor::Vec3(Vec3::ZERO);
                    }
                    if ui.button("Vec4").clicked() {
                        *self = ExprWriterEditor::Vec4(Vec4::ZERO);
                    }
                    ui.separator();
                    ui.menu_button("Prebuilt", |ui| {
                        if ui.button("Random Normalized Vector").clicked() {
                            *self = ExprWriterEditor::Operator(Box::new(
                                ExprOperatorEditor::Normalized(ExprWriterEditor::Operator(
                                    Box::new(ExprOperatorEditor::Subtract(
                                        ExprWriterEditor::Operator(Box::new(
                                            ExprOperatorEditor::Multiply(
                                                ExprWriterEditor::RandVec3,
                                                ExprWriterEditor::Float(2.0),
                                            ),
                                        )),
                                        ExprWriterEditor::Float(1.0),
                                    )),
                                )),
                            ))
                        }
                    });
                    if app.expr_clipboard.is_some() {
                        ui.separator();
                        if ui.button("From clipboard").clicked() {
                            *self = app.expr_clipboard.clone().unwrap().clone();
                        }
                    }
                });
            }
        }
    }
}
