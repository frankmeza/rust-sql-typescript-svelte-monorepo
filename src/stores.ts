import { readable, writable } from "svelte/store"

interface Mailbox {
	messages: string[]
}

class Mailbox {
	constructor() {
		this.messages = []
	}
}

const createMailBox = () => {
	const { subscribe, set, update } = writable(new Mailbox())

	const handleAddMsg = (newMsg: string) =>
		update((mailbox) => ({
			messages: [...mailbox.messages, newMsg],
		}))

    return {
		subscribe,
		addMsg: (newMsg: string) => handleAddMsg(newMsg),
		// getMsg: () => update((n) => n - 1),
		reset: () => set(new Mailbox()),
	}
}

export const mailbox = createMailBox()
// export const readableMailbox = readable(mailbox, function start()
