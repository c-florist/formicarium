import { getWorldState } from "$lib/core/query";
import type { WorldDto } from "@formicarium/domain";
import { writable } from "svelte/store";

export const worldStore = writable<WorldDto | null>(null);

let intervalId: NodeJS.Timeout | null = null;

export const startWorldUpdates = () => {
  if (intervalId) {
    return;
  }

  const setWorldState = async () => {
    const worldData = await getWorldState();
    if (worldData) {
      worldStore.set(worldData);
    }
  };

  setWorldState();

  intervalId = setInterval(setWorldState, 100);
};

export const stopWorldUpdates = () => {
  if (intervalId) {
    clearInterval(intervalId);
    intervalId = null;
  }
};
