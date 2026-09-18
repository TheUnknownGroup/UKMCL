<script>
     import ConfirmDeletePopUp from "$lib/components/ConfirmDeletePopUp.svelte";
     let {children} = $props();
</script>

<style>
     :root {
          position: relative;
          background-color: rgb(0, 0, 0);
          min-height: 100vh;
     }
     :root:before {
          content: "";
          position: fixed;
          inset: 0;
          opacity: 0.7;
          filter: blur(10px);
          z-index: -1;
     }
</style>

{@render children?.()}
<ConfirmDeletePopUp />