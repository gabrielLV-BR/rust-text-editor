<script lang="ts">
  import { join } from "@tauri-apps/api/path";
  import { fs, dialog, event } from "@tauri-apps/api";
  import { BaseDirectory } from "@tauri-apps/api/fs";
  import { onDestroy, onMount } from "svelte";

  type File = {
    filename: string;
    isDir: boolean;
  };

  let path: null | string = null;
  let files: File[] = [];

  let events: event.UnlistenFn[] = [];

  async function update() {
    if(path == null) return;
    const listOfFiles = await fs.readDir(path);
    files = listOfFiles.map((f) => ({ filename: f.name, isDir: !!f.children }));
  }

  async function newFile() {
    if (path == null) return;
    fs.writeFile(await join(path, "novo.txt"), "OI PORRA").catch(console.error);
    update();
  }

  async function load() {
    const folder = await dialog.open({
      directory: true,
      multiple: false,
    });
    path = [...folder].join("");
    update();
  }

  onMount(async () => {
    try {
      events.push(await event.listen("new_file", newFile));
      events.push(await event.listen("new_folder", load));
      events.push(await event.listen("open_folder", load));
      events.push(await event.listen("save", load));
    } catch (e) {
      console.error("Error when setting up event: ", e);
    }
  });

  onDestroy(() => {});

  export let name: string;
</script>

<main>
  <h1>Hello {name}!</h1>
  <p>Caminho: {path}</p>
  <button on:click={load}>Carregar</button>
  <hr />
  <p>Files:</p>
  <ul>
    {#each files as file}
      <li>
        {#if file.isDir}
          DIR -
        {/if}
        {file.filename}
      </li>
    {/each}
  </ul>
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
