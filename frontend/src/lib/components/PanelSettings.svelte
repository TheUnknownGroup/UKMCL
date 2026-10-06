<script>
     import Panel from '$lib/components/Panel.svelte';
     import MainText from '$lib/components/MainText.svelte';
     import { spawn, mins, maxs, rams } from '$lib/js/setts.js';
     import { auth } from "$lib/js/login.js";
     import { invoke } from "@tauri-apps/api/core";
     import { onMount } from 'svelte';

     let min = $state('');
     let max = $state('');
     let inp1 = $state('');
     let inp2 = $state('');
     let form;

     onMount(async () => {
          let num1 = await mins();
          let num2 = await maxs();
          min = num1;
          max = num2;
     });

     let canSave = $derived(inp1.trim() !== '' || inp2.trim !== '');

     function parseOrNull(s) {
          const t = s.trim();
          if (t === '') return null;
          const n = Number(t);
          return Number.isFinite(n) ? n : null;
     }

     async function micro() {
          console.log("launch");
          const result = await auth();
          if (result === 'done') {
               console.log('signed in');
          } else {
               console.log('cancelled');
          }
     }

     async function onSubmit(e) {
          e.preventDefault();
          
          const has1 = inp1.trim() !== '';
          const has2 = inp2.trim() !== '';

          const m = parseOrNull(inp1);
          const M = parseOrNull(inp2);
          
          const args = {};
          if (m !== null) args.min = m;
          if (M !== null) args.max = M;
          
          await rams(args);
          
          let num1 = await mins();
          let num2 = await maxs();
          form?.reset();
          min = num1;
          max = num2;
          inp1 = '';
          inp2 = '';
     }
     
     let { children } = $props();
</script>

<style>
     #img {
          background-image: url("/assets/images/microsoft.png");
          background-size: 204px;
          background-position: center;
     }
</style>

<div class="flex">
     <Panel/>
     <MainText>
          <div class="p-3 border m-1.25 rounded-lg bg-[rgba(0,0,0,0.4)] text-white w-46.75 justify-center float-left">
               <form bind:this={form} onsubmit={onSubmit}>
                    <h3 class="text-center text-xl font-semibold">Java Config</h3>
                    <label for="minimum" class=""> Minimum Amount</label><br>
                    <input type="text" class="bg-[#d9d9d9] text-black rounded-[5px] p-1.75 mb-1.75 mr-1.75 text-[16px]" id="minimum" placeholder="Current: {min}" bind:value={inp1}><br>
                    <label for="maximum" class=""> Maximum Amount</label><br>
                    <input type="text" class="bg-[#d9d9d9] text-black rounded-[5px] p-1.75 mb-1.75 mr-1.75 text-[16px]" id="maximum" placeholder="Current: {max}" bind:value={inp2}>
                    <button type="submit" class="float-right bg-green-600 px-2 pt-0.5 rounded-lg w-full cursor-pointer"> Save </button>
               </form>
          </div>
          <div class="p-3 border m-1.25 rounded-lg bg-[rgba(0,0,0,0.4)] text-white w-46.75 justify-center float-right">
               <h3 class="text-center text-xl font-semibold">Login</h3>
               <button onclick={micro} class="px-2 pt-0.5 mb-1 rounded-lg cursor-pointer w-full" id="img">Microsoft</button><br>
               <button onclick={spawn} class="bg-green-900 px-2 pt-0.5 rounded-lg cursor-pointer w-full">Offline</button>
          </div>
     </MainText>
     {@render children?.()}
</div>