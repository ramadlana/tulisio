use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("invalid note path: {0}")]
    InvalidNotePath(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub vault_path: String,
    pub notes_folder: String,
    pub assets_folder: String,
    pub naming_strategy: String,
    pub auto_cleanup_assets: bool,
}

#[derive(Debug)]
pub struct NoteContext {
    pub assets_dir: PathBuf,
    pub assets_dir_relative: PathBuf,
}

pub fn resolve_note_context(settings: &Settings, note_path: &str) -> Result<NoteContext, StorageError> {
    let note_path = PathBuf::from(note_path);
    let file_name = note_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| StorageError::InvalidNotePath(note_path.to_string_lossy().to_string()))?;

    let date_prefix = file_name
        .split('-')
        .take(3)
        .collect::<Vec<_>>();

    if date_prefix.len() != 3 {
        return Err(StorageError::InvalidNotePath(file_name.to_string()));
    }

    let date = date_prefix.join("-");
    let year = &date[0..4];

    if !date.chars().all(|c| c.is_ascii_digit() || c == '-') {
        return Err(StorageError::InvalidNotePath(file_name.to_string()));
    }

    let assets_dir_relative = Path::new(&settings.assets_folder)
        .join(year)
        .join(date);
    let assets_dir = Path::new(&settings.vault_path).join(&assets_dir_relative);

    Ok(NoteContext {
        assets_dir,
        assets_dir_relative,
    })
}

pub fn vault_absolute_path(settings: &Settings, relative: &Path) -> PathBuf {
    Path::new(&settings.vault_path).join(relative)
}

pub fn ensure_parent_dir(path: &Path) -> Result<(), StorageError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}

pub fn unique_asset_name(prefix: &str, extension: &str) -> String {
    let clean_ext = extension.trim_start_matches('.');
    format!("{}_{}.{}", prefix, Uuid::new_v4(), clean_ext)
}

pub fn normalize_relative_path(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}
