import webSocket from "isomorphic-ws"

export const ws = new webSocket("ws://127.0.0.1:8088/ws/")

ws.onopen = () => {
	ws.send(Date.now())
	console.log("connected")
}

ws.onclose = () => {
	console.log("disconnected")
}

ws.onmessage = (data) => {
	console.log(
		`Roundtrip time: ${Date.now() - parseInt(data.data.toString())} ms`,
	)
}
