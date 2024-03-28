<script>
	import { invoke }  from "@tauri-apps/api/tauri"
  	import Greet from './lib/Greet.svelte'
  	import NetworkPage from "./lib/NetworkPage.svelte"

  	let currentWindow = "HOME";


	
</script>

<main class="container">
	<div id="navpanel">
  		<h1 style="padding: 1rem">Forestry</h1>
		<button class="page-button" on:click={() => {currentWindow = "HOME"}}>HOME</button>
		<button class="page-button" on:click={() => {currentWindow = "NETWORK"}}>Network</button>
		<button class="page-button" on:click={async () => {console.log(await invoke("get_networks", {}))}}>Debug</button>
	</div>

	<div>
  		{#if currentWindow == "HOME"}
  	  		<div>
  	  			Forestry is a general settings manager for Linux targeting users who do no use complete desktop environments such as KDE or Gnome
  	  		</div>

		{:else if currentWindow == "NETWORK"}
			<NetworkPage/>
  		{/if}
  	</div>
</main>

<style>
	.container {
		display: flex;
		height: 100vh;
		margin: 0;
		padding: 0;
	}

	#navpanel {
		display: flex;
		flex-direction: column;
		padding: 0;
		border-right: 1px solid black;
	}

	.page-button {
		margin: 0;
		border-radius: 0;
		border: 0px;
		padding: 1rem 0rem;
		border-bottom: 1px solid black;
	}

</style>
