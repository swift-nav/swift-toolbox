<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri'
	import { register, unregister, unregisterAll } from '@tauri-apps/api/globalShortcut'
	import { WebglPlot, WebglLine, ColorRGBA } from "webgl-plot"
	import { onMount } from 'svelte'

	import { Convert, Message, TcpRequest, TrackingSignalsStatus } from './Message';
	import { LruList, LruNode } from './Lru';

	import * as parseColor from 'parse-color';

	import { serialize, deserialize } from "bson";

	function sendLogMessage(msg: string) {
		invoke('write_log_message', { message: msg })
	}

	async function fetchIpcMessage() {
		return await invoke('fetch_ipc_message');
	}

	function onRightClick() {
		invoke('write_log_message', { message: "right click disabled" })
		return true
	}

	function onReload() {
		invoke('write_log_message', { message: "refresh disabled" })
		return true
	}

	function onPrint() {
		invoke('write_log_message', { message: "print disabled" })
		return true
	}

	const PIKSI_HOST: string = "piksi-relay-bb9f2b10e53143f4a816a11884e679cf.ce.swiftnav.com";
	const PIKSI_PORT: number = 55555;

	function filterNull(obj: object): object {
		return Object.fromEntries(Object.entries(obj).filter(([_, v]) => v != undefined));
	}

	function createRequest(): Array<number>
	{
		const msg: Message = new Message();
		const tcpConnect: TcpRequest = new TcpRequest();

		tcpConnect.host = PIKSI_HOST;
		tcpConnect.port = PIKSI_PORT;

		msg.TcpRequest = tcpConnect;

		const obj = filterNull(Convert.messageToJson(msg));
		const arr = new Uint8Array(serialize(obj));

		return Array.from(arr);
	}

	function connectToRemote()
	{
		let buf = createRequest();
		invoke('send_ipc_message', { buffer: buf })
	}

	if (performance.navigation.type == performance.navigation.TYPE_RELOAD) {
		unregisterAll()
	}

	function onFocus() {
		register("CmdOrControl+R", onReload)
		register("CmdOrControl+P", onPrint)
	}

	function onBlur() {
		unregister("CmdOrControl+R")
		unregister("CmdOrControl+P")
	}

	onMount(async () => {

		const canvas = <HTMLCanvasElement> document.getElementById("my_canvas");
		const devicePixelRatio = window.devicePixelRatio || 1;

		canvas.width = canvas.clientWidth * devicePixelRatio;
		canvas.height = canvas.clientHeight * devicePixelRatio;

		const wglp = new WebglPlot(canvas);
		let trackingStatusUpdates: TrackingSignalsStatus[] = [];

		const numX = 200;

		wglp.gScaleY = 2;
		wglp.gOffsetY = -1;

		const lru = new LruList<WebglLine>();
		const lineMap = new Map<String, LruNode<WebglLine>>();
		const poppedLines: WebglLine[] = [];

		function update() {

			let minX = Infinity;
			let maxX = -Infinity;

			if (trackingStatusUpdates.length == 0) {
				return;
			}
			const trackingStatus = trackingStatusUpdates.shift();
			const lineCount = trackingStatus.data.length;
			for (let idx = 0; idx < lineCount; idx++) {

				const points = trackingStatus.data[idx];
				const label = trackingStatus.labels[idx];

				if (!lineMap.has(label)) {

					const parsedColor = parseColor.default(trackingStatus.colors[idx])
					const lineColor = new ColorRGBA(parsedColor.rgb[0]/256, parsedColor.rgb[1]/256, parsedColor.rgb[2]/256, 1);

					let line: WebglLine;

					if (poppedLines.length != 0) {
						line = poppedLines.pop();
						line.color = lineColor;
					} else {
						line = new WebglLine(lineColor, numX);
						line.constY(undefined);
					}

					wglp.addLine(line);

					const lruNode = lru.push(line);
					lineMap.set(label, lruNode);
				}

				const lruNode = lineMap.get(label);
				const line = lru.access(lruNode);

				for (let point of points) {
					minX = Math.min(minX, point.x);
					maxX = Math.max(maxX, point.x);
				}

				let startIdx = numX - points.length;
				for (let i = numX - 1; i >= startIdx; i--) {
					const pidx = i - startIdx;
					let y = (points[pidx].y - 15) / 45;
					line.setY(i, y);
					let x = 1 - ((((points[pidx].x - minX) % (numX/2)) / (numX/4)));
					line.setX(i, x);
				}
			}

			if (trackingStatus.labels.length < lru.count) {
				const count = lru.count - trackingStatus.labels.length;
				const popped = lru.pop(count);
				poppedLines.push(...popped);
			}
		}

		function newFrame() {
  			update();
  			wglp.update();
			fetchIpcMessage().then((ipc) => {
				if (ipc !== null) {
					const kind = ipc[0];
					if (kind === 1) {
						const buffer = new Uint8Array(ipc[1]);
						const data = deserialize(buffer);
						const msg = Convert.toMessage(data);
						if (msg.TrackingSignalsStatus !== undefined) {
							trackingStatusUpdates.push(msg.TrackingSignalsStatus);
						}
					}
				}
			});
  			requestAnimationFrame(newFrame);
		}

		requestAnimationFrame(newFrame);
	});
</script>

<button on:click={() => sendLogMessage("hello")}>
	Send a log message
</button>

<button on:click={fetchIpcMessage}>
	Fetch an IPC message
</button>

<button on:click={connectToRemote}>
	Connect
</button>

<canvas style="width: 100%; height: 90%" id="my_canvas"></canvas>

<svelte:window
	on:focus={onFocus}
	on:blur={onBlur}
	/>

<svelte:body
	on:contextmenu|preventDefault={onRightClick}
	/>