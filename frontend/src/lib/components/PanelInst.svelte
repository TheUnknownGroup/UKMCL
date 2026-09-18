<script>
     import Panel from '$lib/components/Panel.svelte';
     import MainText from '$lib/components/MainText.svelte';
     import InstanceList from '$lib/components/InstanceList.svelte';

     let { children } = $props();
</script>

<div class="flex">
     <Panel/>
     <MainText>
          <InstanceList/>
     </MainText>
     {@render children?.()}
</div>