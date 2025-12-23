# MyNotesVault

Filesystem-first macOS note-taking app for Apple Silicon using Tauri (Rust) and React + TipTap.

## Vault layout

```
MyNotesVault/
  notes/
    YYYY/
      YYYY-MM-DD-title.md
  assets/
    YYYY/
      YYYY-MM-DD/
        img_<UUID>.png
        file_<UUID>.<ext>
  .appconfig/
    settings.json
```

## Backend commands

| Command | Responsibility |
| --- | --- |
| `save_note` | Write Markdown to disk and optionally clean unused assets |
| `save_image` | Persist pasted image data in the note asset folder |
| `save_attachment` | Copy an arbitrary file into the note asset folder |
| `cleanup_unused_assets` | Remove unreferenced assets for a note |
| `open_file` | Open an asset with the OS default app |

## Frontend behavior

- Loads Markdown into TipTap.
- Edits in WYSIWYG mode.
- Autosaves Markdown (debounced).
- Handles image paste/drop and file attachments.
- Clicking attachments opens them with the default macOS app.

## Example settings

See `examples/MyNotesVault/.appconfig/settings.json`.

## Development

### Run in dev mode (Terminal or VS Code)

1. Install dependencies:

   ```bash
   npm install
   ```

2. Start the Tauri dev app (from Terminal or VS Code integrated terminal):

   ```bash
   npm run tauri dev
   ```

### Build a macOS Apple Silicon app

1. Install dependencies:

   ```bash
   npm install
   ```

2. Build the production app bundle:

   ```bash
   npm run tauri build
   ```

The built `.app` bundle will be located in `src-tauri/target/release/bundle/macos/`.
