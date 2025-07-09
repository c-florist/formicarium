import type { World } from "@formicarium/core";
import { readable } from "svelte/store";
import { browser } from "$app/environment";

const WS_URL = import.meta.env["VITE_WEBSOCKET_BASE_URL"];

export const worldStore = readable<World | null>(null, (set) => {
  if (!browser) {
    return;
  }

  const url = `${WS_URL}/ws/world`;
  const ws = new WebSocket(url);

  ws.onmessage = (event) => {
    const worldData: World = JSON.parse(event.data);
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
