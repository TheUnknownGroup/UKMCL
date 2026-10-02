<script>
     import { onMount } from 'svelte';
     import { setup } from '$lib/js/java.js';
     import CreateBtn from '$lib/components/CreateBtn.svelte';
     import Account from './Account.svelte';

     let { children } = $props();
     
     let container;
     let isEmpty = $state(false);
     let marginLeft = $state('18vw');
     let translate = $state('586%');
     let width = $state('85%');
     
     onMount(() => {
          const update = () => {
               marginLeft = window.innerHeight > 600 ? '32vw' : "18vw";
               translate = window.innerHeight > 600 ? '1114%' : '586%';
               width = window.innerHeight > 600 ? '96.6%' : '94.1%';
          };
          window.addEventListener('resize', update);
          update();

          let cleanup;
          setup(container, (empty) => {
               isEmpty = empty;
          })
          .then((dispose) => { cleanup = dispose; }).catch((e) => console.error(e));
          
          return () => { window.removeEventListener('resize', update), cleanup?.()};
     });
</script>

<div class="p-[0.3125rem_0] justify-start">
     <CreateBtn/>
     <div class="absolute" style:transform={`translateY(${translate})`} style:width={width}>
          <Account />
     </div>
     <div class="grid pt-1.25 gap-1.25 grid-cols-[repeat(auto-fit,minmax(196px,198px))]" bind:this={container}></div>

     {#if isEmpty}
          <p style:margin-left={marginLeft} class="select-none absolute text-center border-2 border-white rounded-[5px] bg-black/40 text-white p-7.5 w-114.75">No instances yet. <br><br> To create one, press the button above this. <br><br> If you made an instance, it will take some time as the app automatically downloads the assets as soon as the instance is made. </p>
     {/if}
     {@render children?.()}
</div>