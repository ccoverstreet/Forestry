<script>
	import { invoke } from "@tauri-apps/api/tauri"
  	import { onMount } from "svelte"

	let displays = [];
	let current_display = null;

	async function getDisplayConfiguration() {
		const res = await invoke("get_display_configuration", {});
		console.log(res);
		displays = res;
	}


	let svgElement;
	let selectedElement = null;


	function makeDraggable(event) {
		console.log(event);
		event.addEventListener("mousedown", startDrag);
		event.addEventListener("mousemove", drag);
		event.addEventListener("mouseup", stopDrag);
		event.addEventListener('mouseleave', stopDrag);
	}

	function getMousePosition(event) {
		var CTM = svgElement.getScreenCTM();
		return {
			x: (event.clientX - CTM.e) / CTM.a,
			y: (event.clientY - CTM.f) / CTM.d
		};
	}

	function startDrag(event) {
		const isPrimary = displays[event.target.parentNode.dataset.index].is_primary;
		if (isPrimary) return;

		console.log(event);

		selectedElement = event.target.parentNode;
	}

	function drag(event) {
		if (!selectedElement) return;

		const pos = getMousePosition(event);

		const bbox = selectedElement.children[0].getBBox();

		const x = pos.x - bbox.width / 2; 
		const y = pos.y - bbox.height / 2; 

		selectedElement.children[0].setAttribute("x", x)
		selectedElement.children[0].setAttribute("y", y + 0)
		selectedElement.children[1].setAttribute("x", x+50)
		selectedElement.children[1].setAttribute("y", y+200)
	}

	function stopDrag(event) {
		selectedElement = null;
	}

	function setDisplayConfiguration() {
		for (const child of svgElement.children) {
			const rect = child.children[0];
			console.log(rect.getBBox(), rect.dataset.name);
		}


	}

	onMount(() => {
		getDisplayConfiguration();
	})
</script>



<div class="setting-page">
	<svg bind:this={svgElement} viewbox="-2000 -2000 6000 6000"style="width: 50%; height: 50%;">
		{#each displays as display, index}
			<g style="cursor: move; pointer-events: bounding-box;" use:makeDraggable={makeDraggable} data-index={index}>
				<rect x="{display.x}" y="{display.y}" width="{display.sizes[display.selected_size].width}" 
		 	 							 	 height="{display.sizes[display.selected_size].height}" fill="red"
		 	 							 							  on:click={() => { current_display = index; console.log(index) }}
		 	 							 							  data-name={display.display_name}>
		 		</rect>
		 		<text x={display.x+50} y="{display.y + 200}" font-size="200px">{display.display_name}</text>
		 	</g>
		{/each}
	</svg>

	<button on:click={getDisplayConfiguration}>TEST</button>
	<button on:click={setDisplayConfiguration}></button>

	{#if current_display != null}
		<div>
			{JSON.stringify(displays[current_display])}
		</div>
	{/if}

</div>


