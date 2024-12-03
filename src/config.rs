use clap::ArgMatches;
use config_file::FromConfigFile;
use serde::Deserialize;
use std::path::Path;
use std::path::PathBuf;

#[derive(Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub depth: Option<usize>,
    pub files0_from: Option<String>,
}

impl Config {
    pub fn get_files_from(&self, options: &ArgMatches) -> Option<String> {
        let from_file: Option<&String> = options.get_one::<String>("files0_from");
        match from_file {
            None => self.files0_from.as_ref().map(|x| x.to_string()),
            Some(x) => Some(x.to_string()),
        }
    }
    pub fn get_depth(&self, options: &ArgMatches) -> usize {
        if let Some(v) = options.get_one("depth") {
            return *v;
        }
        self.depth.unwrap_or(usize::MAX)
    }
}

fn get_config_locations(base: &Path) -> Vec<PathBuf> {
    vec![
        base.join(".dust.toml"),
        base.join(".config").join("dust").join("config.toml"),
    ]
}

pub fn get_config() -> Config {
    if let Some(home) = directories::BaseDirs::new() {
        for path in get_config_locations(home.home_dir()) {
            if path.exists() {
                if let Ok(config) = Config::from_config_file(path) {
                    return config;
                }
            }
        }
    }
    Config {
        ..Default::default()
    }
}