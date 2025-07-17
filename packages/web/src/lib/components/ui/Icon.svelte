<script lang="ts">
let {
  name,
  class: className = "",
}: {
  name: string;
  class?: string;
} = $props();
</script>

<img
  src={`/ui/icons/${name}.png`}
  alt={`${name} icon`}
  class={`inline-block ${className}`}
/>
