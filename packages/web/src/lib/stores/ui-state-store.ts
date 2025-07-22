import { writable } from "svelte/store";

type UIState = {
  showStatsOverlay: boolean;
  selectedAntId: number | null;
};

export const uiStateStore = writable<UIState>({
  showStatsOverlay: false,
  selectedAntId: null,
});

export const setSelectedAntId = (id: number | null) => {
  uiStateStore.update((state) => ({ ...state, selectedAntId: id }));
};

export const deselectAnt = () => {
  setSelectedAntId(null);
};

export const toggleStatsOverlay = () => {
  uiStateStore.update((state) => ({
    ...state,
    showStatsOverlay: !state.showStatsOverlay,
  }));
};
