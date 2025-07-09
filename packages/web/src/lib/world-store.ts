import type { World } from "@formicarium/core";
import { readable } from "svelte/store";

const WS_URL = import.meta.env["VITE_WEBSOCKET_BASE_URL"];

export const world = readable<World | null>(null, (set) => {
  const ws = new WebSocket(`${WS_URL}/ws/world`);

  ws.onmessage = (event) => {
    const worldData = JSON.parse(event.data);
    set(worldData);
  };

  ws.onerror = (error) => {
    console.error("WebSocket error:", error);
    set(null);
  };

  ws.onclose = () => {
    console.log("WebSocket connection closed");
    set(null);
  };

  return () => {
    ws.close();
  };
});
