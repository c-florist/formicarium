import type { WorldDto } from "@formicarium/domain";
import { writable } from "svelte/store";

export const worldStore = writable<WorldDto | null>(null);
