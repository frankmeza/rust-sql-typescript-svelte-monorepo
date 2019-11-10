import webSocket from "isomorphic-ws"

export const ws = new webSocket("ws://127.0.0.1:8088/ws/")
export let mailBox: string[] = []

ws.onopen = () => {
	ws.send(Date.now())
	console.log("connected")
}

ws.onclose = () => {
	console.log("disconnected")
}

ws.onmessage = (data: webSocket.MessageEvent) => {
	const payload = data.data.toString()

	const printMsg = !!parseInt(payload)
		? `Roundtrip time: ${Date.now() - parseInt(data.data.toString())} ms`
		: payload

    console.log(printMsg)
    mailBox = [...mailBox, printMsg] // todo write to store
}
