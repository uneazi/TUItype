use std::fs;
use std::path::PathBuf;
use anyhow::Result;
use directories::ProjectDirs;
use crate::models::AppConfig;

pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let proj_dirs = ProjectDirs::from("", "", "TypingTUI")
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
        
        let config_dir = proj_dirs.config_dir();
        fs::create_dir_all(config_dir)?;
        
        let config_path = config_dir.join("config.toml");
        
        Ok(Self { config_path })
    }

    pub fn load(&self) -> Result<AppConfig> {
        if !self.config_path.exists() {
            // Create default config
            let default = AppConfig::default();
            self.save(&default)?;
            return Ok(default);
        }

        let content = fs::read_to_string(&self.config_path)?;
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self, config: &AppConfig) -> Result<()> {
        let toml_str = toml::to_string_pretty(config)?;
        fs::write(&self.config_path, toml_str)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn path(&self) -> &PathBuf {
        &self.config_path
    }
}
