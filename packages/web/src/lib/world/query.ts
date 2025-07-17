import type { StatsDto } from "@formicarium/domain";
import { invoke } from "@tauri-apps/api/core";

export const getWorldStatistics = async () => {
  try {
    return await invoke<StatsDto>("get_world_statistics");
  } catch (error) {
    console.error("Failed to get world statistics:", error);
  }
};
