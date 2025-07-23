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
  uiState.showWorldStatsPanel = !uiState.showWorldStatsPanel;
};

export const toggleHelpPanel = () => {
  uiState.showHelpPanel = !uiState.showHelpPanel;
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
