<script>
     import Panel from '$lib/components/Panel.svelte';
     import MainText from '$lib/components/MainText.svelte';
     import ListMods from '$lib/components/ListMods.svelte';

     let { children } = $props();
</script>

<div class="flex">
     <ListMods />
     {@render children?.()}
</div>