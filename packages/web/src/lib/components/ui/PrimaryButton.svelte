<script lang="ts">
let { onClick, text }: { onClick: (event?: Event) => void; text: string } =
  $props();
</script>

<button
  onclick={onClick}
  class="bg-green-700 px-6 py-3 font-bold text-white border-b-4 border-green-900 hover:bg-green-800 rounded-sm transition-all duration-150 ease-in-out active:translate-y-0.5 cursor-pointer"
>
  {text}
</button>
