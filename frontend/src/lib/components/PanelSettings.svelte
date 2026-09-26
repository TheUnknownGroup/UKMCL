<script>
     import Panel from '$lib/components/Panel.svelte';
     import MainText from '$lib/components/MainText.svelte';

     let { children } = $props();
</script>

<div class="flex">
     <Panel/>
     <MainText>
          <div>
                <input type="text" class="bg-[#d9d9d9] text-black rounded-[5px] p-1.75 mb-1.75 mr-1.75 text-[16px]" id="java-minimum-size" placeholder="Minimum" required>
                <input type="text" class="bg-[#d9d9d9] text-black rounded-[5px] p-1.75 mb-1.75 mr-1.75 text-[16px]" id="java-maximum-size" placeholder="Maximum" required>
          </div>
     </MainText>
     {@render children?.()}
</div>