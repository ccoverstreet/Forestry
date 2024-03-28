<script>
	import { invoke } from "@tauri-apps/api/tauri"
  	import { onMount } from "svelte"

	let networks = []

	async function getNetworks() {
		console.log("Getting networks...");
		networks = []
		networks = await invoke("get_networks", {});
		networks = networks.concat(networks);
	}

	async function connectNetwork(bssid) {
		console.log(bssid);
		const res = await invoke("connect_network", {bssid})
		getNetworks();
	}


	onMount(() => {
		getNetworks();

		return
	})

</script>

<div id="network-page">
	NETWORK PAGE


	<button on:click={getNetworks}>Get networks</button>

	<div>
		<div class="network-item">
			<p class="network-ssid">SSID</p>
			<p class="network-chan">Frequency</p>
			<p class="network-signal">Signal</p>
			<p class="network-bssd">BSSID</p>
		</div>
	</div>

	<div class="network-list">
		{#if networks.length == 0}
			<p>Retrieving network information...</p>
		{:else}
			{#each networks as net}
				<div class="network-item {net.connected ? "network-active" : ""}"
						on:click={connectNetwork(net.bssid)}>
					<p class="network-ssid">{net.ssid}</p>
					{#if net.chan >= 1 && net.chan <= 14}
						<p class="network-chan">2.4GHz</p>
					{:else}
						<p class="network-chan">5.0Ghz</p>
					{/if}

					{#if net.signal > 75}
						<p class="network-signal" style="color: green">{net.signal}</p>
					{:else if net.signal > 50}
						<p class="network-signal" style="color: yellow">{net.signal}</p>
					{:else}
						<p class="network-signal" style="color: red">{net.signal}</p>
					{/if}

					<p class="network-bssid">{net.bssid}</p>
				</div>
			{/each}
		{/if}
	</div>
</div>

<style>
	#network-page {
		padding: 1rem;
		overflow: scroll;
		height: calc(100vh - 2rem);
	}

	.network-item {
		display: flex;
		padding: 0.25rem;
		margin: 0;
	}

	.network-item:hover {
		background-color: #dddddd;
	}

	.network-item > p {
		margin: 0;
	}

	.network-active {
		background-color: green;
	}

	.network-ssid {
		width: 34ch;
	}

	.network-chan {
		width: 10ch;
	}

	.network-signal {
		width: 8ch;
	}

	.network-bssid {
		width: 18ch;
	}

</style>
