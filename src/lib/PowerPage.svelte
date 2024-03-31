<script>
	import { invoke } from "@tauri-apps/api/tauri"
	import { onMount } from "svelte";
	
	let profiles = [];
	
	async function getPowerProfiles() {
		profiles = await invoke("get_power_profiles", {});
		console.log(profiles);
	}

	async function setPowerProfile(profile) {
		await invoke("set_power_profile", {profile});
		getPowerProfiles();
	}

	onMount(() => {
		getPowerProfiles();
	}) 
</script>

<div id="container">
	<div style="display: flex; align-items: center; font-weight: bold; font-size: 1.5rem;">
		<div>Power Profile</div>
	</div>

	<hr>

	<div class="power-profile">
		<div class="power-profile-name">Profile Name</div>
		<div class="power-profile-driver">Driver</div>
	</div>

	<hr>

	{#each profiles as profile}
		<div class="power-profile {profile.is_active ? "power-profile-active" : "h"}" 
			on:click={setPowerProfile(profile)}>
			<div class="power-profile-name">{profile.name}</div>
			<div class="power-profile-driver">{profile.driver}</div>
		</div>
	{/each}
</div>

<style>
	#container {
		display: block;
		max-width: 40ch;
		position: relative;
		padding: 1rem;
		overflow: scroll;
		height: calc(100vh - 2rem);
	}

	.power-profile {
		display: flex; 
		padding: 0.25rem;
		margin: 0;
		width: 100%;
	}

	.power-profile:hover {
		background-color: var(--color-01);
		color: var(--foreground);
	}

	.power-profile-active {
		color: var(--color-01);
		background-color: var(--color-03);
	}

	.power-profile-name {
		width: 20ch;
	}

	.power-profile-driver {
		width: 20ch;
	}
</style>
