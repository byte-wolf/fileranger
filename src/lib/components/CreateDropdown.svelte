<script lang="ts">
  import { PlusIcon, FileIcon, FolderIcon } from "@lucide/svelte";

  interface CreateDropdownProps {
    onCreateFile: () => void;
    onCreateFolder: () => void;
  }

  let { onCreateFile, onCreateFolder }: CreateDropdownProps = $props();

  let isOpen = $state(false);
  let dropdownRef = $state<HTMLDivElement>();

  function toggleDropdown() {
    isOpen = !isOpen;
  }

  function handleCreateFile() {
    isOpen = false;
    onCreateFile();
  }

  function handleCreateFolder() {
    isOpen = false;
    onCreateFolder();
  }

  function handleClickOutside(event: MouseEvent) {
    if (dropdownRef && !dropdownRef.contains(event.target as Node)) {
      isOpen = false;
    }
  }

  $effect(() => {
    if (isOpen) {
      document.addEventListener("click", handleClickOutside);
      return () => {
        document.removeEventListener("click", handleClickOutside);
      };
    }
  });
</script>

<div class="relative" bind:this={dropdownRef}>
  <button
    onclick={toggleDropdown}
    class="bg-white rounded-full p-2 cursor-pointer transition active:scale-90 hover:shadow flex items-center gap-1"
  >
    <PlusIcon class="size-5 text-sky-500" />
    <span class="text-sky-500 leading-5 pb-0.5">Create</span>
  </button>

  {#if isOpen}
    <div
      class="absolute top-full right-0 mt-1 bg-white rounded-2xl shadow-lg border border-neutral-200 min-w-40 z-10"
    >
      <button
        onclick={handleCreateFile}
        class="w-full flex items-center gap-3 px-4 py-3 text-left hover:bg-sky-50 transition first:rounded-t-2xl last:rounded-b-2xl cursor-pointer"
      >
        <FileIcon class="size-5 fill-sky-100 text-sky-300" />
        <span class="text-neutral-700">New File</span>
      </button>
      <button
        onclick={handleCreateFolder}
        class="w-full flex items-center gap-3 px-4 py-3 text-left hover:bg-sky-50 transition first:rounded-t-2xl last:rounded-b-2xl cursor-pointer border-t border-neutral-100"
      >
        <FolderIcon class="size-5 fill-sky-300 text-sky-300" />
        <span class="text-neutral-700">New Folder</span>
      </button>
    </div>
  {/if}
</div>
