type UIState = {
  showStatsOverlay: boolean;
  showWorldStatsPanel: boolean;
  selectedAntId: number | null;
};

export const uiState = $state<UIState>({
  showStatsOverlay: false,
  showWorldStatsPanel: false,
  selectedAntId: null,
});

export const toggleStatsOverlay = () => {
  uiState.showStatsOverlay = !uiState.showStatsOverlay;
};

export const toggleWorldStatsPanel = () => {
  uiState.showWorldStatsPanel = !uiState.showWorldStatsPanel;
};
