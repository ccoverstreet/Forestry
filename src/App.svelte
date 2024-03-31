<script>
	import { invoke }  from "@tauri-apps/api/tauri"
  	import Greet from './lib/Greet.svelte'
  	import NetworkPage from "./lib/NetworkPage.svelte"
  	import PowerPage from "./lib/PowerPage.svelte"
  	import DisplayPage from "./lib/DisplayPage.svelte"
	import { onMount } from "svelte"


	onMount(async () => {
		const colors = await invoke("default_colors", {});

		for (const [key, value] of Object.entries(colors)) {
			console.log(key.replace("_", "-"), value);
			document.documentElement.style.setProperty("--" + key.replace("_", "-"), value)
		}
		console.log(colors);
	})

  	let currentWindow = "HOME";
</script>
		

<main class="container">
	<div id="navpanel">
  		<h1 style="padding: 1rem;border-bottom: 1px solid var(--color-08); margin: 0; padding: 2rem 1rem;
  		background-color: var(--color-01)">Forestry</h1>
		<button class="page-button" on:click={() => {currentWindow = "HOME"}}>Home</button>
		<button class="page-button" on:click={() => {currentWindow = "NETWORK"}}>Network</button>
		<button class="page-button" on:click={() => {currentWindow = "POWER"}}>Power</button>
		<button class="page-button" on:click={() => {currentWindow = "DISPLAY"}}>Display</button>
	</div>

	<div style="flex-grow: 1;">
  		{#if currentWindow == "HOME"}
  			<div style="display: flex; justify-content: center; align-items: center; height: 100%; flex-direction: column;">
  				<div style="font-size: 3rem; font-weight: bold;">Welcome to Forestry</div>
  	  			<div style="font-size: 2rem; padding: 2rem; line-height: 3rem;">
  	  				Forestry is a general settings manager for Linux targeting users who do no use complete desktop environments such as KDE or Gnome. 
					<br>
					<br>
  	  				Forestry uses the same color scheming convention as terminal color schemes (see Gogh on GitHub), which makes it easy to match the theme of Forestry to your terminal configuration.
  	  			</div>
  	  		</div>

		{:else if currentWindow == "NETWORK"}
			<NetworkPage/>
		{:else if currentWindow == "POWER"}
			<PowerPage/>
		{:else if currentWindow == "DISPLAY"}
			<DisplayPage/>
  		{/if}
  	</div>
</main>

<style>
	.container {
		display: flex;
		height: 100vh;
		margin: 0;
		padding: 0;
		flex-direction: row;
		color: var(--foreground);
	}

	#navpanel {
		display: flex;
		flex-direction: column;
		padding: 0;
		border-right: 1px solid var(--color-08);
		background-color: var(--color-01);
	}

	.page-button {
		margin: 0;
		border-radius: 0;
		border: 0px;
		padding: 1rem 0rem;
		/*border-bottom: 1px solid var(--color-08);*/
		background-color: var(--color-01);
		color: var(--foreground);
		font-weight: bold;
		font-size: 1.25rem;
	}

	.page-button:hover {
		background-color: var(--color-09)
	}

</style>
