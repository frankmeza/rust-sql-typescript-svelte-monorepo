<script lang="ts">
    import { ws } from "./ws_client"
    import { createPerson } from "utils/app_utils"
    // constants
    const nil: null = null // the ts linter throws a first false error
    const BUTTON_TEXT: string = "add record"

    // variables
    let name = ""

    // async handlers
    const handleClick = async () => {
        response = await createPerson(name)
        ws.send(JSON.stringify(`${name} was created`))
    }
</script>

<h2>Add A Person</h2>

<input bind:value={name} placeholder="enter name">
<button on:click={handleClick}>{BUTTON_TEXT}</button>

<p>Hello {name || 'stranger'}!</p>