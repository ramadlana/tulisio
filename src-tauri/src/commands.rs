use std::collections::HashSet;
use std::path::{Path, PathBuf};

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde::{Deserialize, Serialize};

use crate::markdown::extract_asset_paths;
use crate::storage::{
    ensure_parent_dir, normalize_relative_path, resolve_note_context, unique_asset_name,
    vault_absolute_path, Settings, StorageError,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveNoteRequest {
    pub settings: Settings,
    pub note_path: String,
    pub markdown: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveImageRequest {
    pub settings: Settings,
    pub note_path: String,
    pub base64: String,
    pub extension: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveAttachmentRequest {
    pub settings: Settings,
    pub note_path: String,
    pub base64: String,
    pub original_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanupRequest {
    pub settings: Settings,
    pub note_path: String,
    pub markdown: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenFileRequest {
    pub settings: Settings,
    pub relative_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveImageResponse {
    pub relative_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveAttachmentResponse {
    pub relative_path: String,
    pub display_name: String,
}

#[tauri::command]
pub fn save_note(request: SaveNoteRequest) -> Result<(), String> {
    let note_absolute = vault_absolute_path(&request.settings, Path::new(&request.note_path));
    ensure_parent_dir(&note_absolute).map_err(|err| err.to_string())?;
    std::fs::write(&note_absolute, request.markdown.as_bytes()).map_err(|err| err.to_string())?;

    if request.settings.auto_cleanup_assets {
        cleanup_unused_assets(CleanupRequest {
            settings: request.settings,
            note_path: request.note_path,
            markdown: request.markdown,
        })?;
    }

    Ok(())
}

#[tauri::command]
pub fn save_image(request: SaveImageRequest) -> Result<SaveImageResponse, String> {
    let context = resolve_note_context(&request.settings, &request.note_path)
        .map_err(|err| err.to_string())?;

    std::fs::create_dir_all(&context.assets_dir).map_err(|err| err.to_string())?;

    let file_name = unique_asset_name("img", &request.extension);
    let file_path = context.assets_dir.join(&file_name);
    let image_bytes = STANDARD
        .decode(&request.base64)
        .map_err(|err| err.to_string())?;

    std::fs::write(&file_path, image_bytes).map_err(|err| err.to_string())?;

    let relative_path = normalize_relative_path(&context.assets_dir_relative.join(file_name));

    Ok(SaveImageResponse { relative_path })
}

#[tauri::command]
pub fn save_attachment(request: SaveAttachmentRequest) -> Result<SaveAttachmentResponse, String> {
    let context = resolve_note_context(&request.settings, &request.note_path)
        .map_err(|err| err.to_string())?;

    std::fs::create_dir_all(&context.assets_dir).map_err(|err| err.to_string())?;

    let extension = Path::new(&request.original_name)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("bin");

    let file_name = unique_asset_name("file", extension);
    let destination = context.assets_dir.join(&file_name);

    ensure_parent_dir(&destination).map_err(|err| err.to_string())?;
    let bytes = STANDARD
        .decode(&request.base64)
        .map_err(|err| err.to_string())?;
    std::fs::write(&destination, bytes).map_err(|err| err.to_string())?;

    let relative_path = normalize_relative_path(&context.assets_dir_relative.join(file_name));

    Ok(SaveAttachmentResponse {
        relative_path,
        display_name: request.original_name,
    })
}

#[tauri::command]
pub fn cleanup_unused_assets(request: CleanupRequest) -> Result<(), String> {
    let context = resolve_note_context(&request.settings, &request.note_path)
        .map_err(|err| err.to_string())?;
    let referenced = normalized_reference_set(&request.settings, request.markdown.as_str());

    if !context.assets_dir.exists() {
        return Ok(());
    }

    for entry in std::fs::read_dir(&context.assets_dir).map_err(|err| err.to_string())? {
        let entry = entry.map_err(|err| err.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            continue;
        }

        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| StorageError::InvalidNotePath("invalid asset name".to_string()))
            .map_err(|err| err.to_string())?;

        let relative = normalize_relative_path(&context.assets_dir_relative.join(file_name));

        if !referenced.contains(&relative) {
            std::fs::remove_file(&path).map_err(|err| err.to_string())?;
        }
    }

    Ok(())
}

#[tauri::command]
pub fn open_file(request: OpenFileRequest) -> Result<(), String> {
    let requested = PathBuf::from(&request.relative_path);
    let absolute = if requested.is_absolute() {
        requested
    } else {
        vault_absolute_path(&request.settings, &requested)
    };

    if !absolute.exists() {
        return Err("File not found".to_string());
    }

    open::that(absolute).map_err(|err| err.to_string())?;
    Ok(())
}

fn normalized_reference_set(settings: &Settings, markdown: &str) -> HashSet<String> {
    let references = extract_asset_paths(markdown);
    references
        .into_iter()
        .map(|reference| {
            let path = Path::new(&reference);
            if path.is_absolute() {
                if let Ok(stripped) = path.strip_prefix(&settings.vault_path) {
                    normalize_relative_path(stripped)
                } else {
                    normalize_relative_path(path)
                }
            } else {
                normalize_relative_path(path)
            }
        })
        .collect()
}
