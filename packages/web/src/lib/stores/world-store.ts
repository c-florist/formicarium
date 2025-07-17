import type { WorldDto } from "@formicarium/domain";
import { invoke } from "@tauri-apps/api/core";
import { writable } from "svelte/store";

export const worldStore = writable<WorldDto | null>(null);

let intervalId: NodeJS.Timeout | null = null;

export const startWorldUpdates = () => {
  if (intervalId) {
    return;
  }

  const fetchWorldState = async () => {
    try {
      const worldData = await invoke<WorldDto | null>("get_world_state");
      if (worldData) {
        worldStore.set(worldData);
      }
    } catch (error) {
      console.error("Failed to get world state:", error);
    }
  };

  fetchWorldState();

  intervalId = setInterval(fetchWorldState, 100);
};

export const stopWorldUpdates = () => {
  if (intervalId) {
    clearInterval(intervalId);
    intervalId = null;
  }
};
