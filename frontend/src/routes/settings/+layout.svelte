<script>
     import LoginMicro from '$lib/components/LoginMicro.svelte';
     let {children} = $props();
</script>

{@render children?.()}
<LoginMicro />