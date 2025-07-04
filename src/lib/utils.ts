import type { DirEntry } from "@tauri-apps/plugin-fs";

export function isHidden(entry: DirEntry) {
  return entry.name.startsWith(".") || entry.name.startsWith("$");
}
