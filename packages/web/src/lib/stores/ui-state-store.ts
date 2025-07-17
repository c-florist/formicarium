import { writable } from "svelte/store";

type UIState = {
  showStatsOverlay: boolean;
};

export const uiStateStore = writable<UIState>({
  showStatsOverlay: false,
});
