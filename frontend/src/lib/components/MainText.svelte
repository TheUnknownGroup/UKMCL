<script lang="ts">
     let { children } = $props();
</script>

<div class="flex-1 overflow-auto ml-12.5 mt-[31px]">
     {@render children?.()}
</div>