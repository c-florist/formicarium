type SimulationState = {
  speed: number;
  isPaused: boolean;
};

export const simulationState = $state<SimulationState>({
  speed: 150,
  isPaused: false,
});

const SPEED_INCREMENT = 10;
const SPEED_LIMIT = {
  min: 300,
  max: 20,
};

export const incrementSpeed = () => {
  simulationState.speed = Math.min(
    simulationState.speed - SPEED_INCREMENT,
    SPEED_LIMIT.min,
  );
};

export const decrementSpeed = () => {
  simulationState.speed = Math.max(
    simulationState.speed + SPEED_INCREMENT,
    SPEED_LIMIT.max,
  );
};

export const resetSimulationSpeed = () => {
  simulationState.speed = 150;
};

export const togglePause = () => {
  simulationState.isPaused = !simulationState.isPaused;
};
