use serde::{Deserialize, Serialize};
use std::fs;
use std::error::Error;

//
// ─────────────────────────────────────────────────────────────
//   GAME CONFIG
// ─────────────────────────────────────────────────────────────
//

#[derive(Debug, Deserialize, Serialize)]
pub struct GameConfig {
    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub author: String,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            name: "Untitled Game".into(),
            description: "No description provided".into(),
            author: "Unknown Author".into(),
        }
    }
}

//
// ─────────────────────────────────────────────────────────────
//   ASSETS CONFIG
// ─────────────────────────────────────────────────────────────
//

#[derive(Debug, Deserialize, Serialize)]
pub struct AssetsConfig {
    #[serde(default)]
    pub sb3: String,

    #[serde(default)]
    pub icon: String,

    #[serde(default)]
    pub banner: String,

    #[serde(default)]
    pub audio: String,
}

impl Default for AssetsConfig {
    fn default() -> Self {
        Self {
            sb3: "project.sb3".into(),
            icon: "icon.png".into(),
            banner: "banner.png".into(),
            audio: "audio.wav".into(),
        }
    }
}

//
// ─────────────────────────────────────────────────────────────
//   ROOT PROJECT CONFIG
// ─────────────────────────────────────────────────────────────
//

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectConfig {
    #[serde(default)]
    pub game: GameConfig,

    #[serde(default)]
    pub assets: AssetsConfig,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            game: GameConfig::default(),
            assets: AssetsConfig::default(),
        }
    }
}

//
// ─────────────────────────────────────────────────────────────
//   LOADING + SAVING
// ─────────────────────────────────────────────────────────────
//

impl ProjectConfig {
    /// Load a TOML project file and apply defaults to missing fields.
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
        let mut config: ProjectConfig = toml::from_str(&content)?;

        // Merge defaults
        let defaults = ProjectConfig::default();

        // Game defaults
        if config.game.name.is_empty() {
            config.game.name = defaults.game.name;
        }
        if config.game.description.is_empty() {
            config.game.description = defaults.game.description;
        }
        if config.game.author.is_empty() {
            config.game.author = defaults.game.author;
        }

        // Asset defaults
        if config.assets.sb3.is_empty() {
            config.assets.sb3 = defaults.assets.sb3;
        }
        if config.assets.icon.is_empty() {
            config.assets.icon = defaults.assets.icon;
        }
        if config.assets.banner.is_empty() {
            config.assets.banner = defaults.assets.banner;
        }
        if config.assets.audio.is_empty() {
            config.assets.audio = defaults.assets.audio;
        }

        Ok(config)
    }

    /*
    /// Save the project back to TOML.
    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let toml_string = toml::to_string_pretty(self)?;
        fs::write(path, toml_string)?;
        Ok(())
    }
     */
}
