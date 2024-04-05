<script>
	import { invoke } from "@tauri-apps/api/tauri"

	export let displayInfo = null;
	export let displayUpdateHandler = () => {};

	async function setDisplayMode(displayName, modeName) {
		console.log(displayName, modeName);
		const res = await invoke("set_display_mode", {displayName: displayName, modeName: modeName})

		setTimeout(displayUpdateHandler, 500);
	}
</script>

{#if displayInfo != null}
	<div class="display-info-container">
		<div>{displayInfo.display_name} {#if displayInfo.is_primary}<span><b>Primary</b></span>{/if}</div>
		{#if displayInfo.is_connected}
			<div>Connected</div>
		{:else}
			<div>Disconnected</div>
		{/if}

		{#each displayInfo.sizes as size, index}
			<div class="size-info {displayInfo.selected_size == index ? "size-info-selected" : ""}" on:click={setDisplayMode(displayInfo.display_name, size.name)}>
				<div style="min-width: 15ch;">{size.name}</div>
				<div>{size.width}x{size.height}</div>
			</div>
		{/each}
	</div>
{/if}

<style>

	.display-info-container {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.size-info {
		display: flex;
		gap: 1rem;
		border-radius: 0.25rem;
		padding: 0.25rem;
	}

	.size-info:hover {
		background-color: var(--color-01);
	}

	.size-info-selected {
		background-color: var(--color-03);
		color: var(--color-01);
	}
</style>
