<script>
     import Panel from '$lib/components/Panel.svelte';
     import MainText from '$lib/components/MainText.svelte';
     import Card from '$lib/components/Card.svelte';
     let { children } = $props();
</script>

<div class="flex">
     <Panel />
     <MainText>
          <Card />
     </MainText>
     {@render children?.()}
</div>