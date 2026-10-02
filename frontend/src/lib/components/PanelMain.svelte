<script>
     import Panel from '$lib/components/Panel.svelte';
     import MainText from '$lib/components/MainText.svelte';
     import Card from '$lib/components/Card.svelte';
     import Account from './Account.svelte';
     import { conf } from '$lib/js/start';
     import { onMount } from 'svelte';

     let show = $state(false);

     onMount(async () => {
          let wait = await conf();
          if (wait == true) {
               show = true;
          }
     });
     
     let { children } = $props();
</script>

<div class="flex">
     <Panel />
     <MainText>
          <Card />
          {#if show == true}
               <Account />
          {/if}
     </MainText>
     {@render children?.()}
</div>