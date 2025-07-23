import { goto } from "$app/navigation";

type UIState = {
  showStatsOverlay: boolean;
  showWorldStatsPanel: boolean;
  showHelpPanel: boolean;
  selectedAntId: number | null;
  confirmation: {
    isOpen: boolean;
    title: string;
    message: string;
    onConfirm: () => void;
  };
};

export const uiState = $state<UIState>({
  showStatsOverlay: false,
  showWorldStatsPanel: false,
  showHelpPanel: false,
  selectedAntId: null,
  confirmation: {
    isOpen: false,
    title: "",
    message: "",
    onConfirm: () => {},
  },
});

export const toggleStatsOverlay = () => {
  uiState.showStatsOverlay = !uiState.showStatsOverlay;
};

export const toggleWorldStatsPanel = () => {
  const isOpen = uiState.showWorldStatsPanel;
  uiState.showWorldStatsPanel = !isOpen;
  if (!isOpen) {
    uiState.showHelpPanel = false;
  }
};

export const toggleHelpPanel = () => {
  const isOpen = uiState.showHelpPanel;
  uiState.showHelpPanel = !isOpen;
  if (!isOpen) {
    uiState.showWorldStatsPanel = false;
  }
};

export const showConfirmation = (
  title: string,
  message: string,
  onConfirm: () => void,
) => {
  uiState.confirmation = {
    isOpen: true,
    title,
    message,
    onConfirm,
  };
};

export const hideConfirmation = () => {
  uiState.confirmation.isOpen = false;
};

export const confirmAndExit = () => {
  showConfirmation(
    "Exit Simulation",
    "Are you sure you want to exit the simulation and return to the main menu?",
    () => {
      goto("/");
      hideConfirmation();
    },
  );
};
