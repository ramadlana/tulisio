import { useEffect, useMemo, useRef, useState } from "react";
import { EditorContent, useEditor } from "@tiptap/react";
import StarterKit from "@tiptap/starter-kit";
import Link from "@tiptap/extension-link";
import Image from "@tiptap/extension-image";
import { Markdown } from "@tiptap/markdown";
import { invoke } from "@tauri-apps/api/tauri";
import type { Settings } from "../config/settings";

const AUTOSAVE_DEBOUNCE_MS = 600;

type NoteEditorProps = {
  initialMarkdown: string;
  notePath: string;
  settings: Settings;
};

type SaveImageResponse = {
  relativePath: string;
};

type SaveAttachmentResponse = {
  relativePath: string;
  displayName: string;
};

export function NoteEditor({ initialMarkdown, notePath, settings }: NoteEditorProps) {
  const [status, setStatus] = useState("Idle");
  const autosaveTimer = useRef<number>();
  const latestMarkdown = useRef(initialMarkdown);

  const editor = useEditor({
    extensions: [
      StarterKit,
      Link.configure({
        openOnClick: false
      }),
      Image,
      Markdown
    ],
    content: initialMarkdown,
    editorProps: {
      handlePaste: (_view, event) => handlePaste(event),
      handleDrop: (_view, event) => handleDrop(event)
    },
    onUpdate: ({ editor }) => {
      const markdown = editor.storage.markdown.getMarkdown();
      latestMarkdown.current = markdown;
      scheduleAutosave(markdown);
    }
  });

  useEffect(() => {
    if (!editor) {
      return;
    }
    editor.commands.setContent(initialMarkdown, "markdown");
  }, [editor, initialMarkdown]);

  const scheduleAutosave = (markdown: string) => {
    if (autosaveTimer.current) {
      window.clearTimeout(autosaveTimer.current);
    }

    autosaveTimer.current = window.setTimeout(() => {
      saveNote(markdown).catch((error) => {
        console.error("Failed to save note", error);
        setStatus("Save failed");
      });
    }, AUTOSAVE_DEBOUNCE_MS);
  };

  const saveNote = async (markdown: string) => {
    setStatus("Saving...");
    await invoke("save_note", {
      settings,
      notePath,
      markdown
    });
    setStatus("Saved");
  };

  const handlePaste = (event: ClipboardEvent) => {
    if (!event.clipboardData) {
      return false;
    }

    const files = Array.from(event.clipboardData.files || []);
    if (files.length === 0) {
      return false;
    }

    void handleFileInsert(files, "image");
    return true;
  };

  const handleDrop = (event: DragEvent) => {
    const files = Array.from(event.dataTransfer?.files || []);
    if (files.length === 0) {
      return false;
    }

    void handleFileInsert(files, "mixed");
    return true;
  };

  const handleFileInsert = async (files: File[], mode: "image" | "mixed") => {
    const images = files.filter((file) => file.type.startsWith("image/"));
    const attachments = mode === "mixed" ? files.filter((file) => !file.type.startsWith("image/")) : [];

    for (const image of images) {
      const arrayBuffer = await image.arrayBuffer();
      const base64 = btoa(String.fromCharCode(...new Uint8Array(arrayBuffer)));
      const response = await invoke<SaveImageResponse>("save_image", {
        settings,
        notePath,
        base64,
        extension: image.name.split(".").pop() || "png"
      });

      const markdown = `![](${response.relativePath})`;
      editor?.chain().focus().insertContent(markdown).run();
    }

    for (const attachment of attachments) {
      const arrayBuffer = await attachment.arrayBuffer();
      const base64 = btoa(String.fromCharCode(...new Uint8Array(arrayBuffer)));
      const response = await invoke<SaveAttachmentResponse>("save_attachment", {
        settings,
        notePath,
        base64,
        originalName: attachment.name
      });

      const markdown = `[📎 ${response.displayName}](${response.relativePath})`;
      editor?.chain().focus().insertContent(markdown).run();
    }
  };

  const handleAttachClick = async () => {
    const filePicker = document.createElement("input");
    filePicker.type = "file";
    filePicker.multiple = true;
    filePicker.onchange = () => {
      const files = Array.from(filePicker.files || []);
      if (files.length > 0) {
        void handleFileInsert(files, "mixed");
      }
    };
    filePicker.click();
  };

  const handleOpenLink = async (event: React.MouseEvent<HTMLDivElement>) => {
    const target = event.target as HTMLElement;
    const link = target.closest("a");
    if (!link) {
      return;
    }

    event.preventDefault();
    const href = link.getAttribute("href");
    if (!href) {
      return;
    }

    await invoke("open_file", {
      settings,
      relativePath: href
    });
  };

  const toolbar = useMemo(() => {
    return (
      <div style={{ display: "flex", gap: "8px", marginBottom: "12px" }}>
        <button type="button" onClick={handleAttachClick}>
          Attach File
        </button>
        <span>{status}</span>
      </div>
    );
  }, [status]);

  return (
    <section>
      {toolbar}
      <div
        style={{
          border: "1px solid #e5e5e5",
          borderRadius: "8px",
          padding: "12px",
          minHeight: "360px"
        }}
        onClick={handleOpenLink}
      >
        <EditorContent editor={editor} />
      </div>
    </section>
  );
}
