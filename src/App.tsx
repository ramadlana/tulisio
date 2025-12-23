import { useMemo, useState } from "react";
import { NoteEditor } from "./editor/NoteEditor";
import { DEFAULT_SETTINGS, Settings } from "./config/settings";

const SAMPLE_NOTE = `# Welcome\n\nThis is a filesystem-first note.\n\n- Paste or drop images\n- Attach files with the button\n`;

export default function App() {
  const [settings] = useState<Settings>(DEFAULT_SETTINGS);
  const [notePath] = useState("notes/2024/2024-05-16-welcome.md");

  const initialMarkdown = useMemo(() => SAMPLE_NOTE, []);

  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div>
          <h1>MyNotesVault</h1>
          <div className="vault-path">{settings.vaultPath}</div>
        </div>
        <div>
          <div className="section-title">Notes</div>
          <div className="note-list">
            <button type="button" className="active">
              2024-05-16 · Welcome
            </button>
            <button type="button">2024-05-12 · Project brief</button>
            <button type="button">2024-05-02 · Design sprint</button>
          </div>
        </div>
      </aside>
      <div className="main-panel">
        <header className="app-header">
          <h2>Welcome</h2>
          <span>Filesystem-first notes</span>
        </header>
        <main className="editor-shell">
          <NoteEditor
            initialMarkdown={initialMarkdown}
            notePath={notePath}
            settings={settings}
          />
        </main>
      </div>
    </div>
  );
}
