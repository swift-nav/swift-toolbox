<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri'
	import { register, unregisterAll } from '@tauri-apps/api/globalShortcut'
	import { WebglPlot, WebglLine, ColorRGBA } from "webgl-plot"
	import { onMount } from 'svelte'

	function sendLogMessage() {
		invoke('write_log_message', { message: "poop hello" })
	}

	async function fetchIpcMessage() {
		const ipcMessage = await invoke('fetch_ipc_message')
		console.log(ipcMessage)
	}

	function onRightClick() {
		invoke('write_log_message', { message: "right click disabled" })
		return false
	}

	function onReload() {
		invoke('write_log_message', { message: "refresh disabled" })
		return false
	}

	function onPrint() {
		invoke('write_log_message', { message: "print disabled" })
		return false
	}

	unregisterAll()

	register("CmdOrControl+R", onReload)
	register("CmdOrControl+P", onPrint)

	onMount(async () => {

		const canvas = <HTMLCanvasElement> document.getElementById("my_canvas");
		const devicePixelRatio = window.devicePixelRatio || 1;

		canvas.width = canvas.clientWidth * devicePixelRatio;
		canvas.height = canvas.clientHeight * devicePixelRatio;

		const numX = canvas.width;
		const color = new ColorRGBA(Math.random(), Math.random(), Math.random(), 1);
		const line = new WebglLine(color, numX);
		const wglp = new WebglPlot(canvas);

		line.arrangeX();
		wglp.addLine(line);

		function update() {

			const freq = 0.001;
			const amp = 0.5;
			const noise = 0.1;

			for (let i = 0; i < line.numPoints; i++) {
				const ySin = Math.sin(Math.PI * i * freq * Math.PI * 2);
				const yNoise = Math.random() - 0.5;
				line.setY(i, ySin * amp + yNoise * noise);
			}
		}

		function newFrame() {
  			update();
  			wglp.update();
  			requestAnimationFrame(newFrame);
		}

		requestAnimationFrame(newFrame);
	});
</script>

<div>
	<canvas style="width: 100%; height: 300px" id="my_canvas"></canvas>
</div>

<button on:click={sendLogMessage}>
	Send a log message
</button>

<button on:click={fetchIpcMessage}>
	Fetch an IPC message
</button>

<svelte:body on:contextmenu|preventDefault={onRightClick} />