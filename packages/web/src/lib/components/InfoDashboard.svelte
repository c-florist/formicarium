<script lang="ts">
let { class: className = "" }: { class?: string } = $props();
</script>

<div
  class={`bg-stone-800 text-white border-4 border-stone-900 p-4 ${className}`}
>
  <h2 class="text-xl font-bold mb-4">Info Dashboard</h2>
  <p>Dashboard content goes here.</p>
</div>
