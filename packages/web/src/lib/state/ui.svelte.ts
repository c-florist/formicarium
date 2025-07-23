type UIState = {
  showStatsOverlay: boolean;
  menuIsOpen: boolean;
  selectedAntId: number | null;
};

export const uiState = $state<UIState>({
  showStatsOverlay: false,
  menuIsOpen: false,
  selectedAntId: null,
});

export const toggleStatsOverlay = () => {
  uiState.showStatsOverlay = !uiState.showStatsOverlay;
};

export const toggleMenu = () => {
  uiState.menuIsOpen = !uiState.menuIsOpen;
};
