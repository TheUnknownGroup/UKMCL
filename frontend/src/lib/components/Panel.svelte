<script>
     let { children } = $props();
</script>

<div class="min-h-full h-auto flex fixed flex-col overflow-y-auto bg-[rgba(0,0,0,0.3)] p-1.25 border-r-black border-transparent border-2">
     <div class="flex flex-col items-center py-px-5 py-px-0 gap-2.5">
          <div class="flex justify-center shrink-0 hover:shadow-[0_0_15px_#CCCCCC] transition duration-300 ease-in-out">
               <a class="cursor-pointer" href="https://github.com/TheUnknownGroup" target="_blank" rel="noopener">
                    <img src="/assets/images/icon.png" alt="Icon" width="45">   
               </a>
          </div>
          <div class="flex justify-center shrink-0">
               <a class="cursor-pointer" href="/">
                    <img src="/assets/images/house-door.svg" alt="Home" width="30">   
               </a>
          </div>
          <div class="flex justify-center shrink-0">
               <a class="cursor-pointer" href="/instances">
                    <img src="/assets/images/list-ul.svg" alt="Instances" width="30">   
               </a>
          </div>
          <div class="flex justify-center shrink-0">
               <a class="cursor-pointer pt-px-5 fixed z-0 bottom-3.75" href="/settings">
                    <img src="/assets/images/gear-wide-connected.svg" alt="Icon" width="30">   
               </a>
          </div>
     </div>
     
     {@render children?.()}
</div>

