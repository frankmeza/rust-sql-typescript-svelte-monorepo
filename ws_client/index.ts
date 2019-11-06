const webSocket = require("ws")

const ws = new webSocket("ws://127.0.0.1:9192")

ws.on("open", function open() {
	ws.send("something")
})

ws.on("message", function incoming(data) {
	console.log(data)
})
