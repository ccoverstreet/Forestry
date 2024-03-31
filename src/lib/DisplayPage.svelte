<script>
	import { invoke } from "@tauri-apps/api/tauri"
  	import { onMount } from "svelte"

	let displays = [];
	let primary_display = null;
	let current_display = null;

	async function getDisplayConfiguration() {
		const res = await invoke("get_display_configuration", {});
		console.log(res);
		displays = [];
		displays = res;
		for (const [i, conf] of Object.entries(res)) {
			if (conf.is_primary) primary_display = i;
		}
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
		console.log("stopDrag handler");

		// Snap windows next to each other

		// This absolutely needs to be rewritten
		// Find primary svg data then reloop for snap
		// Super inefficient and should be restructured
		let primary = null;
		for (const child of svgElement.children) {
			const rect = child.children[0];
			if (rect.dataset.isprimary) {
				primary = rect;
				break;
			}
		}



		for (const child of svgElement.children) {
			const rect = child.children[0];
			const text = child.children[1];
			if (rect.dataset.isprimary === "true") continue;

			const bbox_0 = primary.getBBox();
			const bbox_1 = rect.getBBox();
			console.log(rect.getBBox(), primary.getBBox());

			if (bbox_1.x < bbox_0.x) {
				rect.setAttribute("x", bbox_0.x - bbox_1.width);
				text.setAttribute("x", bbox_0.x - bbox_1.width + 50);
			} else if (bbox_1.x >= bbox_0.x && bbox_1.x < bbox_0.x + bbox_0.width) {
				rect.setAttribute("x", bbox_0.x);
				text.setAttribute("x", bbox_0.x + 50);
			} else if (bbox_1.x >= bbox_0.x + bbox_0.width) {
				rect.setAttribute("x", bbox_0.x + bbox_0.width);
				text.setAttribute("x", bbox_0.x + bbox_1.width + 50);
			}

			if (bbox_1.y < bbox_0.y) {
				rect.setAttribute("y", bbox_0.y - bbox_1.height);
				text.setAttribute("y", bbox_0.y - bbox_1.height + 200);
			} else if (bbox_1.y >= bbox_0.y && bbox_1.y < bbox_0.y + bbox_0.height) {
				rect.setAttribute("y", bbox_0.y);
				text.setAttribute("y", bbox_0.y + 200);
			} else if (bbox_1.y >= bbox_0.y + bbox_0.height) {
				rect.setAttribute("y", bbox_0.y + bbox_0.height);
				text.setAttribute("y", bbox_0.y + bbox_0.height + 200);
			}
		}


		selectedElement = null;
	}

	async function setDisplayConfiguration() {
		const displays = [];
		for (const child of svgElement.children) {
			const rect = child.children[0];
			const name = rect.dataset.name;
			const bbox = rect.getBBox();
			console.log(rect.getBBox(), rect.dataset.name);
			displays.push({
				name: name,
				x: Math.round(bbox.x),
				y: Math.round(bbox.y),
				width: bbox.width,
				height: bbox.height
			});
		}

		console.log(displays);

		const res = await invoke("set_display_configuration", { config: {
			screens: displays
		}});

		getDisplayConfiguration();
	}

	function saveDisplayConfiguration() {
		console.log("TODO");
	}

	onMount(() => {
		getDisplayConfiguration();
	})
</script>



<div class="setting-page">
	<div id="svg-container">
		<svg id="svg-window" bind:this={svgElement} viewbox="-2000 -2000 6000 6000"style="width: 100%; height: 100%;">
			{#each displays as display, index}
				<g style="cursor: move; pointer-events: bounding-box;" use:makeDraggable={makeDraggable} data-index={index}>
					<rect x="{display.x}" y="{display.y}" width="{display.sizes[display.selected_size].width}" 
		 	 							  	  height="{display.sizes[display.selected_size].height}" 
		 	  																fill="{display.is_primary ? "var(--color-07)" : "var(--color-08)"}"
		 	  		 	 stroke="black"
					 	 stroke-width="10"
		 	 							 							  	  on:click={() => { current_display = index; console.log(index) }}
		 	 							 							  	  data-name={display.display_name}
		 	 							 							  	  data-isprimary={display.is_primary}>
		 			</rect>
		 			<text x={display.x+50} y="{display.y + 200}" font-size="200px">{display.display_name}</text>
		 		</g>
			{/each}
		</svg>
	</div>


	<div id="controls" style="display: flex;">
		<div style="flex-grow: 1;"></div>
		<button on:click={getDisplayConfiguration}>TEST</button>
		<button on:click={setDisplayConfiguration}>Apply</button>
		<button on:click={saveDisplayConfiguration}>Save</button>
	</div>

	{#if current_display != null}
		<div>
			{JSON.stringify(displays[current_display])}
		</div>
	{/if}

</div>

<style>
	#svg-container {
		aspect-ratio: 4 / 2;
		max-width: 80ch;
		min-width: 20ch;
		width: 100%;
	}

	#svg-window {
		background-color: var(--color-01);
	}	
	
	#controls {
		max-width: 80ch;
		min-width: 40ch;
		gap: 0.5rem;
	}
</style>
