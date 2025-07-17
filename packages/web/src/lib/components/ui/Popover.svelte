<script lang="ts">
import type { Snippet } from "svelte";

const POPOVER_POSITIONS = {
  TOP: "top",
  BOTTOM: "bottom",
  LEFT: "left",
  RIGHT: "right",
} as const;
type PopoverPosition =
  (typeof POPOVER_POSITIONS)[keyof typeof POPOVER_POSITIONS];

let {
  text,
  position = POPOVER_POSITIONS.TOP,
  class: className = "",
  children,
}: {
  text: string;
  position?: PopoverPosition;
  class?: string;
  children: Snippet;
} = $props();

let visible = $state<boolean>(false);

const positionClasses = {
  top: "bottom-full left-1/2 -translate-x-1/2 mb-2",
  bottom: "top-full left-1/2 -translate-x-1/2 mt-2",
  left: "right-full top-1/2 -translate-y-1/2 mr-2",
  right: "left-full top-1/2 -translate-y-1/2 ml-2",
};

const arrowClasses = {
  top: "left-1/2 -bottom-1 -translate-x-1/2",
  bottom: "left-1/2 -top-1 -translate-x-1/2",
  left: "top-1/2 -right-1 -translate-y-1/2",
  right: "top-1/2 -left-1 -translate-y-1/2",
};

const show = () => {
  visible = true;
};

const hide = () => {
  visible = false;
};
</script>

<div
  class={`relative inline-block ${className}`}
  onmouseenter={show}
  onmouseleave={hide}
  aria-label="popover"
  role="tooltip"
>
  {#if children}
    {@render children()}
  {/if}

  {#if visible}
    <div
      class={`absolute z-10 whitespace-nowrap rounded-md bg-stone-900 px-3 py-1 text-sm text-white shadow-lg ${positionClasses[position]}`}
    >
      {text}
      <div
        class={`absolute h-2 w-2 rotate-45 bg-stone-900 ${arrowClasses[position]}`}
      ></div>
    </div>
  {/if}
</div>
