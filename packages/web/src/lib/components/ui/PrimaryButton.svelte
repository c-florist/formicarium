<script lang="ts">
let { onClick, text }: { onClick: () => void; text: string } = $props();
</script>

<button
  onclick={onClick}
  class="bg-green-700 px-6 py-3 font-bold text-white border-2 rounded-sm border-green-900 shadow-[4px_4px_0_theme(colors.green.900)] hover:bg-green-800 active:translate-y-0.5 active:shadow-[2px_2px_0_theme(colors.green.900)] transition-all cursor-pointer"
>
  {text}
</button>
