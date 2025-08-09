use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use serde::{Deserialize, Serialize};

pub mod controller;
pub mod effect;
pub mod expr;
pub mod helpers;
pub mod modifiers;

use effect::EffectEditor;
use std::io::{self, Read};

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct OmagariProject {
    pub effects: Vec<EffectEditor>,
}

impl OmagariProject {
    pub fn load<P: AsRef<std::path::Path>>(p: P) -> Result<Self, std::io::Error> {
        let mut file = std::fs::File::open(p)?;
        let mut ron_string = String::new();
        file.read_to_string(&mut ron_string)?;
        let graph: OmagariProject = ron::de::from_str(&ron_string)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(graph)
    }
}

impl From<OmagariProject> for OmagariBundle {
    fn from(project: OmagariProject) -> Self {
        Self {
            effects: project
                .effects
                .iter()
                .map(|e| OmagariEffect {
                    texture_asset: "effects/cloud2.png".to_string(),
                    effect: e.produce(),
                })
                .collect(),
        }
    }
}

pub struct OmagariEffect {
    pub texture_asset: String,
    pub effect: EffectAsset,
    // For setting parenting for hanabi
    // is_child: bool,
}

pub struct OmagariBundle {
    pub effects: Vec<OmagariEffect>,
}

pub mod prelude {
    pub use super::OmagariProject;
}

pub mod editor_prelude {
    use std::path::PathBuf;

    pub use super::controller::*;
    pub use super::effect::*;
    pub use super::expr::*;
    pub use super::helpers::*;
    pub use super::modifiers::*;

    pub use super::OmagariProject;

    use super::expr::ExprWriterEditor;

    #[derive(Default)]
    pub struct AppContext {
        pub expr_clipboard: Option<ExprWriterEditor>,
        pub visible_effects: Vec<String>,
        pub filename: Option<PathBuf>,
    }
}
