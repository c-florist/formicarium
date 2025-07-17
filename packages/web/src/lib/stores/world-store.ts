import type { WorldDto } from "@formicarium/domain";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { writable } from "svelte/store";

export const worldStore = writable<WorldDto | null>(null);

async function setupWorld() {
  try {
    const initialState = await invoke<WorldDto>("get_world_state");
    worldStore.set(initialState);
  } catch (error) {
    console.error("Failed to get initial world state:", error);
  }

  await listen("world_update", async () => {
    try {
      const updatedState = await invoke<WorldDto>("get_world_state");
      worldStore.set(updatedState);
    } catch (error) {
      console.error("Failed to get updated world state:", error);
    }
  });
}

setupWorld();
