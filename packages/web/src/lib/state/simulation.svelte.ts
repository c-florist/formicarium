type SimulationState = {
  speed: number;
};

export const simulationState = $state<SimulationState>({ speed: 150 });

const SPEED_INCREMENT = 5;
const SPEED_LIMIT = {
  min: 300,
  max: 25,
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

export const resetSpeed = () => {
  simulationState.speed = 150;
};
