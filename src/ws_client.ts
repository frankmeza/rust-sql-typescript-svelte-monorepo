import webSocket from "isomorphic-ws"
import { mailboxStore } from "./stores"

export const ws = new webSocket("ws://127.0.0.1:8088/ws/")

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

    mailboxStore.addMsg(printMsg)
    console.log(mailboxStore)
}
