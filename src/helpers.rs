use bevy::prelude::*;
use bevy_egui::*;

use crate::editor_prelude::AppContext;

pub trait UiProvider {
    fn draw_ui(&mut self, app: &mut AppContext, ui: &mut egui::Ui, index: u64);
}

pub fn ui_tools_for_expr_writer(label: &str, ui: &mut egui::Ui) -> ExprControl {
    ui.horizontal(|ui| {
        let delete_button = ui.button("❌").on_hover_text("Right-click to delete");
        if delete_button.secondary_clicked() {
            return ExprControl::Delete;
        }

        if ui.button("🗐").clicked() {
            return ExprControl::Copy;
        }

        if !label.is_empty() {
            ui.add_space(5.0); // Optional spacing
            ui.label(label);
        }

        ExprControl::Noop
    })
    .inner
}

pub fn ui_for_f32(ui: &mut egui::Ui, v: f32) -> f32 {
    let mut v = v;
    ui.add(
        egui::DragValue::new(&mut v)
            .speed(0.01)
            .range(-1000.0..=1000.0),
    );
    v
}

pub fn ui_for_f32_ex(ui: &mut egui::Ui, v: f32, min: f32, max: f32, speed: f32) -> f32 {
    let mut v = v;
    ui.add(egui::DragValue::new(&mut v).speed(speed).range(min..=max));
    v
}

pub fn ui_for_u32_ex(ui: &mut egui::Ui, v: u32, min: u32, max: u32, speed: u32) -> u32 {
    let mut v = v;
    ui.add(egui::DragValue::new(&mut v).speed(speed).range(min..=max));
    v
}

pub fn _ui_for_num_ex<T>(ui: &mut egui::Ui, v: T, min: T, max: T, speed: f32) -> T
where
    T: egui::emath::Numeric,
{
    let mut v = v;
    ui.add(egui::DragValue::new(&mut v).speed(speed).range(min..=max));
    v
}

pub fn ui_for_vec3(ui: &mut egui::Ui, mut v: Vec3) -> Vec3 {
    ui.horizontal(|col_ui| {
        for i in 0..3 {
            col_ui.add_space(5.0);
            col_ui.add(
                egui::DragValue::new(&mut v[i])
                    .speed(0.01)
                    .range(-1000.0..=1000.0),
            );
        }
        col_ui.menu_button("xyz", |ui| {
            if ui.button("Vec3::X").clicked() {
                v = Vec3::X;
            }
            if ui.button("Vec3::Y").clicked() {
                v = Vec3::Y;
            }
            if ui.button("Vec3::Z").clicked() {
                v = Vec3::Z;
            }
            if ui.button("Vec3::ZERO").clicked() {
                v = Vec3::ZERO;
            }
            if ui.button("Vec3::ONE").clicked() {
                v = Vec3::ONE;
            }
        });
    });

    v
}

pub fn ui_for_vec4(ui: &mut egui::Ui, mut v: Vec4) -> Vec4 {
    ui.horizontal(|col_ui| {
        for i in 0..4 {
            col_ui.add_space(5.0); // Optional spacing

            col_ui.add(
                egui::DragValue::new(&mut v[i])
                    .speed(0.01)
                    .range(0.0..=100.0),
            );
        }
    });
    v
}

pub fn ui_for_list_item(ui: &mut egui::Ui, index: usize, len: usize) -> Option<ListCommand> {
    if ui
        .button("❌")
        .on_hover_text("Right-click to delete")
        .secondary_clicked()
    {
        return Some(ListCommand::Remove(index));
    }

    if let Some(ret) = ui
        .add_enabled_ui(index > 0, |ui| {
            if ui.button("⬆").clicked() {
                Some(ListCommand::Swap((index, index - 1)))
            } else {
                None
            }
        })
        .inner
    {
        return Some(ret);
    }

    if let Some(ret) = ui
        .add_enabled_ui(index < len - 1, |ui| {
            if ui.button("⬇").clicked() {
                Some(ListCommand::Swap((index, index + 1)))
            } else {
                None
            }
        })
        .inner
    {
        return Some(ret);
    }

    None
}

pub fn unique_collapsing(salt_id: u64, text: &str, ui: &mut egui::Ui) -> egui::CollapsingHeader {
    let blend = format!("{}{}", text, salt_id);
    egui::CollapsingHeader::new(text).id_salt(ui.make_persistent_id(blend))
}

pub enum ExprControl {
    Noop,
    Delete,
    Copy,
}

pub enum ListCommand {
    Remove(usize),
    Swap((usize, usize)),
}

impl ListCommand {
    pub fn apply<T>(&self, list: &mut Vec<T>) {
        match self {
            ListCommand::Remove(i) => {
                list.remove(*i);
            }
            ListCommand::Swap((a, b)) => {
                list.swap(*a, *b);
            }
        }
    }
}
