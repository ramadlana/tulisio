import { useMemo, useState } from "react";
import { NoteEditor } from "./editor/NoteEditor";
import { DEFAULT_SETTINGS, Settings } from "./config/settings";

const SAMPLE_NOTE = `# Welcome\n\nThis is a filesystem-first note.\n\n- Paste or drop images\n- Attach files with the button\n`;

export default function App() {
  const [settings] = useState<Settings>(DEFAULT_SETTINGS);
  const [notePath] = useState("notes/2024/2024-05-16-welcome.md");

  const initialMarkdown = useMemo(() => SAMPLE_NOTE, []);

  return (
    <main style={{ padding: "24px", maxWidth: 960, margin: "0 auto" }}>
      <h1>MyNotesVault</h1>
      <p>Vault: {settings.vaultPath}</p>
      <NoteEditor
        initialMarkdown={initialMarkdown}
        notePath={notePath}
        settings={settings}
      />
    </main>
  );
}
