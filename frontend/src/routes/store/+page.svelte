<script lang="ts">
     import Panel from '#lib/components/Panel.svelte';
     import MainText from '#lib/components/MainText.svelte';
     
     let { children } = $props();
</script>
<div class="flex">
     <MainText>
          <p></p>
     </MainText>
     {@render children?.()}
</div>
