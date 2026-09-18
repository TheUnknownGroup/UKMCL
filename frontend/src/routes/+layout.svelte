<script>
import "../app.css";

let { children } = $props();</script>

<svelte:head>
	<link rel="icon" href="/assets/icon.png" />
</svelte:head>

{@render children?.()}
