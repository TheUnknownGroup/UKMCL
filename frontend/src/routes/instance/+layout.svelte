<script>
     import ConfirmDeletePopUp from "$lib/components/ConfirmDeletePopUp.svelte";
     let {children} = $props();
</script>

<style>
     :root:before {
          filter: blur(10px);
     }
</style>

{@render children?.()}
<ConfirmDeletePopUp />