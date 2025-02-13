use std::collections::HashMap;

use axoasset::SourceFile;
use camino::Utf8PathBuf;
use serde::Deserialize;

use crate::config::analytics::Analytics;
use crate::config::theme::Theme;
use crate::errors::*;
use crate::message::{Message, MessageType};
use crate::site::markdown::SyntaxTheme;

use crate::config::artifacts::Artifacts;

#[derive(Debug, Deserialize)]
pub struct Social {
    pub image: Option<String>,
    pub image_alt: Option<String>,
    pub twitter_account: Option<String>,
}

/// Config for us building and integrating your mdbook
#[derive(Debug, Deserialize)]
pub struct MdBookConfig {
    /// Path to the mdbook
    pub path: String,
    /// Whether to enable the custom oranda/axo theme
    pub theme: Option<bool>,
}

/// Config related to styling your page
#[derive(Debug, Deserialize)]
pub struct StyleConfig {
    pub theme: Theme,
    pub syntax_theme: SyntaxTheme,
    pub additional_css: Vec<String>,
    pub oranda_css_version: Option<String>,
}

impl Default for StyleConfig {
    fn default() -> Self {
        StyleConfig {
            theme: Theme::Dark,
            additional_css: vec![],
            syntax_theme: SyntaxTheme::MaterialTheme,
            oranda_css_version: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct OrandaConfig {
    pub description: Option<String>,
    pub dist_dir: Option<String>,
    pub static_dir: Option<String>,
    pub homepage: Option<String>,
    pub name: Option<String>,
    pub no_header: Option<bool>,
    pub readme_path: Option<String>,
    pub repository: Option<String>,
    pub analytics: Option<Analytics>,
    pub additional_pages: Option<HashMap<String, String>>,
    pub social: Option<Social>,
    pub artifacts: Option<Artifacts>,
    pub version: Option<String>,
    pub logo: Option<String>,
    pub favicon: Option<String>,
    pub path_prefix: Option<String>,
    pub license: Option<String>,
    /// Config for mdbook
    #[serde(alias = "md_book")]
    pub mdbook: Option<MdBookConfig>,
    pub changelog: Option<bool>,
    pub styles: Option<StyleConfig>,
}

impl OrandaConfig {
    pub fn load(config_path: &Utf8PathBuf) -> Result<Option<OrandaConfig>> {
        let msg = format!("Loading config at {}", config_path);
        Message::new(MessageType::Info, &msg).print();
        tracing::info!("{}", &msg);
        let config_result = SourceFile::load_local(config_path.as_path());

        match config_result {
            Ok(config) => {
                let data: OrandaConfig = config.deserialize_json()?;
                tracing::debug!("{:?}", data);
                Ok(Some(data))
            }
            Err(_) => {
                Message::new(MessageType::Info, "No config found, using default values").print();
                Ok(None)
            }
        }
    }
}
