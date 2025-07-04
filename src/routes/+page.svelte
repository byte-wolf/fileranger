<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import {
    readDir,
    BaseDirectory,
    type DirEntry,
    writeTextFile,
    mkdir,
  } from "@tauri-apps/plugin-fs";
  import * as tauri_path from "@tauri-apps/api/path";
  import { onMount } from "svelte";

  import { FolderIcon, FileIcon, ArrowUpIcon } from "@lucide/svelte";
  import { isHidden } from "../lib/utils";
  import CreateDropdown from "../lib/components/CreateDropdown.svelte";

  let home = $state("");
  let path_input = $state("");
  let current_path = $state("");
  let entries: DirEntry[] = $state([]);
  let scrollContainer: HTMLDivElement;

  let sorted_entries: DirEntry[] = $derived(
    entries
      .toSorted((a, b) => {
        if (a.isDirectory && !b.isDirectory) return -1;
        if (!a.isDirectory && b.isDirectory) return 1;
        return a.name.localeCompare(b.name);
      })
      .filter((entry) => !isHidden(entry))
  );

  // Reset scroll position when path changes
  $effect(() => {
    if (scrollContainer && current_path) {
      scrollContainer.scrollTop = 0;
    }
  });

  onMount(async () => {
    home = await tauri_path.homeDir();

    path_input = await tauri_path.join(home);
    current_path = path_input;

    getEntries(current_path);
  });

  async function getEntries(path: string) {
    try {
      entries = await readDir(path, { baseDir: BaseDirectory.AppLocalData });
      current_path = path;
    } catch (error) {
      console.error(error);
      path_input = current_path;
    }

    path_input = await tauri_path.normalize(path_input);
  }

  async function ascendDirectory() {
    path_input = await tauri_path.dirname(path_input);
    getEntries(path_input);
  }

  async function handleEntryClick(entry: DirEntry) {
    if (entry.isDirectory) {
      path_input = await tauri_path.join(path_input, entry.name);
      getEntries(path_input);
    }
  }

  async function handleSubmitFilePath() {
    getEntries(path_input);
  }

  async function handleCreateFile() {
    const fileName = prompt("Enter file name:");
    if (fileName) {
      try {
        const filePath = await tauri_path.join(current_path, fileName);
        await writeTextFile(filePath, "");
        getEntries(current_path); // Refresh the directory listing
      } catch (error) {
        console.error("Error creating file:", error);
        alert("Failed to create file");
      }
    }
  }

  async function handleCreateFolder() {
    const folderName = prompt("Enter folder name:");
    if (folderName) {
      try {
        const folderPath = await tauri_path.join(current_path, folderName);
        await mkdir(folderPath);
        getEntries(current_path); // Refresh the directory listing
      } catch (error) {
        console.error("Error creating folder:", error);
        alert("Failed to create folder");
      }
    }
  }
</script>

<main class="w-full bg-sky-50 h-screen flex flex-col">
  <div class="bg-sky-50 p-4 flex gap-2">
    <button
      onclick={ascendDirectory}
      class="bg-white rounded-full p-2 cursor-pointer transition active:scale-90 hover:shadow"
    >
      <ArrowUpIcon class="size-5 text-sky-500" />
    </button>
    <form onsubmit={handleSubmitFilePath} class="w-full">
      <input
        class="cursor-pointer hover:shadow transition bg-white rounded-2xl px-3 py-1.5 w-full text-sky-500 ring-0 ring-offset-0 outline-0 focus:text-sky-800 font-mono focus:outline-0 focus:ring-2 focus:ring-sky-400"
        type="text"
        name="path"
        id="path"
        bind:value={path_input}
      />
    </form>
    <CreateDropdown
      onCreateFile={handleCreateFile}
      onCreateFolder={handleCreateFolder}
    />
  </div>

  <div
    class="h-full px-4 py-2 bg-white rounded-tl-3xl flex-grow overflow-hidden flex flex-col"
  >
    <table class="w-full table-fixed">
      <thead>
        <tr class="border-b border-neutral-200">
          <th class="text-left py-3 px-1 font-medium text-neutral-700 w-18"
            ><span class="px-3 py-1">Type</span></th
          >
          <th class="text-left py-3 px-1 font-medium text-neutral-700"
            ><span class="px-3 py-1">Name</span></th
          >
        </tr>
      </thead>
    </table>
    <div class="flex-1 overflow-y-auto" bind:this={scrollContainer}>
      <table class="w-full overflow-hidden table-fixed">
        <tbody class="overflow-y-scroll">
          {#each sorted_entries as entry}
            <tr
              class="hover:bg-sky-50 border-b border-neutral-100 cursor-pointer"
              onclick={() => handleEntryClick(entry)}
            >
              <td class="py-3 px-4 w-16">
                {#if entry.isDirectory}
                  <FolderIcon
                    class={"size-5 " +
                      (isHidden(entry)
                        ? "fill-sky-100/30 text-sky-300/30"
                        : "fill-sky-300 text-sky-300")}
                  />
                {:else}
                  <FileIcon
                    class={"size-5 " +
                      (isHidden(entry)
                        ? "fill-sky-100/30 text-sky-300/30"
                        : "fill-sky-100 text-sky-300")}
                  />
                {/if}
              </td>
              <td class="py-3 px-4 text-neutral-700 max-w-0">
                <div
                  class={"text-ellipsis text-nowrap overflow-hidden " +
                    (isHidden(entry) ? "text-neutral-400" : "")}
                >
                  {entry.name}
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
</main>

<style>
</style>
