<script>
	import { invoke } from "@tauri-apps/api/tauri"

	export let hidden = false;
	export let network = {};
	export let refreshHandler = () => {};

	let password = "";

	let returnCode = 0;

	async function connectNetworkPassword(event) {
		event.preventDefault(); 
		returnCode = await invoke("connect_network_password", { network, password });

		if (returnCode == 0) {
			password = "";
			network = {};
			hidden = true;
			refreshHandler();
			return;
		}
	}
</script>

{#if !hidden}
	<div id="mask" on:click={() => {hidden = true}}>
		<div id="container" on:click={(event) => { event.stopPropagation() }}>
			<div>Enter password for <span style="font-weight: bold;">{network.ssid}</span></div>
			<form on:submit={connectNetworkPassword}>
				<input bind:value={password}/>
				<button>Connect</button>
			</form>

			{#if returnCode != 0}
				<p>Invalid credentials provided</p>
			{/if}
		</div>
	</div>
{/if}

<style>
	#mask {
		position: absolute;
		top:0;
		left:0;
		width: 100%;
		height: 100%;
		z-index: 100;
		background-color: rgba(0, 0, 0, 0.5);
	}
	#container {
		position: absolute;
		width: 30ch;
		top: calc(100vh / 2 - 1rem);
		left: calc(100vw / 2 - 15ch);
	}
</style>
