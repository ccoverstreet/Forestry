<script>
  	import NetworkPasswordPrompt from "./NetworkPasswordPrompt.svelte"
	import { invoke } from "@tauri-apps/api/tauri"
  	import { onMount } from "svelte"


	let wifiOn = false;
	let networks = []
	let promptHidden = true;
	let selectedNetwork = {};

	async function getNetworks() {
		console.log("Getting networks...");
		wifiOn = await invoke("is_wifi_on", {});
		if (!wifiOn) {
			networks = [];
			return;
		}


		networks = [];
		networks = await invoke("get_networks", {});
	}

	async function connectNetwork(network) {
		const ssid = network.ssid;
		const isExistingConnection = await invoke("is_existing_connection", { ssid })
		if (isExistingConnection) {
			const res = await invoke("connect_network", {network})
			getNetworks();
		} else {
			console.log("NEED TO PROVIDE PASSWORD");
			promptHidden = false;
			selectedNetwork = network;
		}
	}

	async function enableWifi() {
		const res = await invoke("enable_wifi", {})
		getNetworks();
	}

	async function disableWifi() {
		const res = await invoke("disable_wifi", {})
		getNetworks();
	}

	onMount(() => {
		getNetworks();

		return
	})

</script>

<div id="network-page">

	<div id="wifi-header">
		<div style="display: flex; align-items: center; font-weight: bold; font-size: 1.5rem;">
			<div>Wi-Fi</div>
		</div>

		<div style="flex-grow: 1;"></div>

		<button on:click={getNetworks} style="background-color: var(--color-08); color: var(--color-01)">Refresh</button>

		{#if wifiOn}
			<button on:click={disableWifi} style="background-color: var(--color-02)">Disable</button>
		{:else}
			<button on:click={enableWifi} style="background-color: var(--color-03); color: var(--color-01)">Enable</button>
		{/if}
	</div>

	<NetworkPasswordPrompt refreshHandler={getNetworks} bind:hidden={promptHidden} bind:network={selectedNetwork}/>

	<hr>



	<div>
		<div class="network-item">
			<p class="network-ssid">SSID</p>
			<p class="network-chan">Frequency</p>
			<p class="network-signal">Signal</p>
			<p class="network-bssid">BSSID</p>
		</div>
	</div>

	<div class="network-list">
		{#if networks.length == 0}
			<p>Retrieving network information...</p>
		{:else}
			{#each networks as net}
				{#if net.ssid != "--"}
					<div class="network-item {net.connected ? "network-active" : ""}"
							on:click={connectNetwork(net)}>
						<p class="network-ssid">{net.ssid}</p>
						{#if net.chan >= 1 && net.chan <= 14}
							<p class="network-chan">2.4GHz</p>
						{:else}
							<p class="network-chan">5.0Ghz</p>
						{/if}

						{#if net.signal > 75}
							<p class="network-signal" style="color: var(--color-03)">{net.signal}</p>
						{:else if net.signal > 50}
							<p class="network-signal" style="color: var(--color-04)">{net.signal}</p>
						{:else}
							<p class="network-signal" style="color: var(--color-02)">{net.signal}</p>
						{/if}

						<p class="network-bssid">{net.bssid}</p>
					</div>
				{/if}
			{/each}
		{/if}
	</div>
</div>

<style>
	#network-page {
		position: relative;
		padding: 1rem;
		overflow: scroll;
		height: calc(100vh - 2rem);
	}

	.network-item {
		display: flex;
		padding: 0.25rem;
		margin: 0;
		width: 100%;
	}

	.network-item:hover {
		background-color: var(--color-01);
	}

	.network-item > p {
		margin: 0;
	}

	.network-active {
		color: var(--color-01);
		background-color: var(--color-03);
	}

	.network-ssid {
		min-width: 34ch;
	}

	.network-chan {
		min-width: 10ch;
	}

	.network-signal {
		min-width: 8ch;
	}

	.network-bssid {
		min-width: 20ch;
	}

	#wifi-header {
		display: flex;
		gap: 1rem;
		align-items: center;
	}


</style>
