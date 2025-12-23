export type Settings = {
  vaultPath: string;
  notesFolder: string;
  assetsFolder: string;
  namingStrategy: "uuid" | "timestamp";
  autoCleanupAssets: boolean;
};

export const DEFAULT_SETTINGS: Settings = {
  vaultPath: "/Users/USER/Documents/MyNotesVault",
  notesFolder: "notes",
  assetsFolder: "assets",
  namingStrategy: "uuid",
  autoCleanupAssets: true
};
