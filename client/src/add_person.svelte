<script lang="ts">
    import { ws } from "./ws_client"
    import { createPerson } from "utils/app_utils"
    import { constants } from "./constants"

    const { ADD_RECORD } = constants

    let name: string = ""
    let response: string = ""

    const handleClick = async () => {
        response = await createPerson(name)
        console.log(response)
    }
</script>

<h2>Add A Person</h2>

<input bind:value={name} placeholder="enter name" />
<button on:click={handleClick}>{ADD_RECORD(name)}</button>

<p>Hello {name || 'anonymous'}!</p>
