<script>
     import { onMount } from 'svelte';
     import { getInst, deletes, launch, edit } from '$lib/js/insts';

     let name = $state('Unknown');

     onMount(() => {
          name = getInst();
          document.title = name;
     });

     const onDelete = () =>  deletes(name);
     const onLaunch = () => launch(name);
     const onEdit = () => edit(name);
</script>

<div>
     <h1>{name}</h1>
     <div id="card" class="flex gap-2 mt-auto flex-1">
          <button onclick={onLaunch} class="cursor-pointer flex-1 p-[8px_15px] rounded-[5px] shadow-[0_0_10px_rgba(0,0,0,0.4)] text-white transition-colors duration-300 ease-in-out bg-[#008000] hover:bg-[#00a600]"><img src="/assets/images/play.svg" alt="Launch"></button>
          <button onclick={onDelete} class="cursor-pointer flex-1 p-[8px_15px] rounded-[5px] shadow-[0_0_10px_rgba(0,0,0,0.4)] text-white transition-colors duration-300 ease-in-out bg-[#a80000] hover:bg-[#ff0000]"><img src="/assets/images/trash.svg" alt="Delete"></button>
          <button onclick={onEdit} class="cursor-pointer flex-1 p-[8px_15px] rounded-[5px] shadow-[0_0_10px_rgba(0,0,0,0.4)] text-white transition-colors duration-300 ease-in-out bg-[#FFA500] hover:bg-[#FFBB38]" ><img src="/assets/images/tools.svg" alt="Edit"></button>
     </div>
</div>
