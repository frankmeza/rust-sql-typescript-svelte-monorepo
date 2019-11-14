import { readable, writable } from "svelte/store"

// export interface MailboxInterface {
//     messages: string[]
// }

// export class Mailbox {
//     messages: string[]

// 	constructor() {
// 		this.messages = []
// 	}
// }

function createMailBox() {
    // let mailbox = new Mailbox()
	const { subscribe, set, update } = writable([""])

	const handleAddMsg = (newMsg: string) =>
		update((mailbox) => [...mailbox, newMsg])

    return {
		subscribe,
		addMsg: (newMsg: string) => handleAddMsg(newMsg),
		// getMsg: () => update((n) => n - 1),
		reset: () => set([""]),
	}
}

export const mailbox = createMailBox()
// export const readableMailbox = readable(mailbox, function start()
